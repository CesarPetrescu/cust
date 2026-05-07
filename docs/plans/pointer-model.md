# Cust Pointer Model Design

Last updated: 2026-05-04

## Goal

Add a small, safe pointer subset that fits Cust's interpreter architecture without trying to become a full C implementation. The first implementation should support ordinary address/dereference use cases, pointer parameters, and array/pointer interaction while preserving deterministic diagnostics and avoiding host-memory access.

## Non-goals for the first pointer milestone

- No raw host addresses.
- No function pointers, casts, `malloc`, or standard-library calls.
- No `void *`.

## Syntax subset

### Types

Cust should keep the current scalar storage model (`int`/`char` values are `i64`) and add one pointer level for scalar and array element references:

```c
int *p = &x;
char *p = &c;
int *p = values;
char *p = "abc";
struct Point *p = &point;
int first(int *values) { return values[0]; }
void set_x(struct Point *p, int x) { p->x = x; }
```

Recommended initial parser representation:

```rust
enum TypeName {
    Int,
    Char,
}

enum ValueKind {
    Scalar,
    Array(usize),
    Pointer { pointee: TypeName },
}
```

Do not encode pointee type semantics deeply at first; use it for diagnostics and obvious type-shape checks only.

### Expressions

Add:

- Address-of: `&name`, `&array[index]`, `&struct_value.field`, `&nested.inner.field`, `&struct_array[index]`, and `&struct_array[index].field` for supported scalar/struct targets
- Dereference: `*ptr`
- Pointer indexing: `ptr[index]`
- Struct pointer field access: `ptr->field` and `(*ptr).field` for scalar fields on supported struct pointers
- Struct pointer fields: `struct Node { int *external; struct Node *next; };` store interpreter-owned pointer metadata, including self-referential struct pointer fields and const-pointee metadata for `const T *field`.
- Null literal: use integer `0` as the only null pointer literal in pointer initializers/assignments and pointer arguments.

`&` should be parsed as a unary operator with precedence matching the existing unary operators. `*` remains multiplication in binary position and becomes dereference in unary position. Parser context already distinguishes unary/binary `-`; apply the same approach for unary `*`.

### Statements

Add pointer declarations and assignments:

```c
int *p = &x;
*p = 4;
p = &y;
p = 0;
```

The first implementation should not support declarations like `int *p;` because Cust currently requires scalar declarations to initialize values.

## Runtime representation

Add interpreter-owned pointer targets instead of raw addresses:

```rust
enum PointerValue {
    Null,
    Scalar { scope_id: usize, name: String },
    ArrayElement { array: Rc<RefCell<ArrayValue>>, index: usize },
    ArrayBase { array: Rc<RefCell<ArrayValue>> },
    Struct { scope_id: usize, name: String },
    StructElement { scope_id: usize, name: String, index: usize },
    StructField { scope_id: usize, name: String, element_index: Option<usize>, fields: Vec<String> },
}
```

Store variables as a value enum rather than separate scalar/array maps:

```rust
enum RuntimeValue {
    Scalar(i64),
    Array(Rc<RefCell<ArrayValue>>),
    Pointer(PointerValue),
}
```

This can be introduced incrementally by first wrapping the existing scalar and array storage, then adding pointer variants.

### Scope/lifetime safety

Pointers must not silently reference variables that are out of scope. Use monotonically increasing `scope_id`s for block/function scopes and store the scope id in scalar pointers. When a scope is popped, mark its id dead. Dereferencing a pointer to a dead scope should fail with:

```text
pointer to out-of-scope variable '<name>'
```

Array storage is already reference counted; an array pointer can keep storage alive as long as the pointer is alive. This is acceptable for the safe subset and matches the current array-parameter sharing model.

### Read-only storage

String literals already create read-only array storage. Pointer writes through a pointer derived from a string literal must report the existing style of read-only diagnostic, for example:

```text
cannot modify read-only array '<name>'
```

If the pointer has no source name (for example a temporary string literal argument), use:

```text
cannot modify read-only array through pointer
```

### Null and invalid pointer diagnostics

Dereferencing or indexing null should fail deterministically:

```text
null pointer dereference
```

Pointer indexing beyond an array should reuse array bounds wording where possible:

```text
array pointer index 4 out of bounds for length 4
```

Dereferencing an array-base pointer directly should read/write element `0` only if the pointer was formed from an array-to-pointer decay. Prefer requiring explicit `p[0]` in the first milestone if that keeps implementation simpler; record the final choice in the implementation PR/status update.

## Array/pointer interaction

Supported first:

- Passing an array variable to an `int *`/`char *` parameter decays to an array-base pointer.
- Passing a supported struct/union array variable to a matching aggregate pointer parameter (`struct T *` / `union T *`) decays to an interpreter-owned pointer to element zero.
- Passing a string literal to a `char *` parameter decays to a read-only array-base pointer.
- Struct pointer fields copy pointer values by value during struct copy/parameter/return flows; they do not clone or own the pointee.
- `const T *` pointer fields preserve pointee constness for conversion/write checks, while `T * const` pointer fields reject field reassignment.
- `ptr[index]` reads/writes array storage when the pointer target is array-base.
- `&array[index]` creates an array-element pointer.
- `*(&array[index])` reads/writes exactly that element.
- Array-backed scalar pointers and struct/union-array element pointers support scoped pointer arithmetic: `p + n`, `p - n`, `p += n`, `p -= n`, `p++`/`--p`, and pointer difference for two pointers into the same interpreter-owned array. Resulting positions must remain within an actual element; one-past pointers remain outside Cust's supported subset.

Defer generic scalar/standalone struct pointer arithmetic, one-past pointers, and pointer ordering until separate milestones justify them.

## Implementation steps

1. Refactor parser type parsing from `expect_type()` into a typed helper that returns base type plus pointer depth/array shape while preserving existing diagnostics for `int`, `char`, arrays, and parameters.
2. Add AST variants for pointer declarations, pointer assignment, address-of, dereference, and pointer-index expressions.
3. Refactor interpreter storage to a single `RuntimeValue` enum while preserving all current valid/invalid fixture behavior.
4. Implement pointer declarations and `&scalar` / `*ptr` reads.
5. Implement dereference assignments (`*ptr = expr;`) and scalar pointer re-assignment.
6. Implement pointer parameters and array/string literal decay to pointer arguments.
7. Implement pointer indexing and `&array[index]`.
8. Add fixtures with explicit expected results for the supported pointer subset after Cust behavior is green. Where a fixture is also valid supported C, optionally compare Cust output against GCC/Clang/cc as an external oracle only; Cust itself remains the execution engine.

## Acceptance tests

### Valid fixtures

`tests/fixtures/valid/pointers_scalars.c`

```c
int main() {
    int x = 3;
    int y = 4;
    int *p = &x;
    *p = *p + 2;
    p = &y;
    return x * 10 + *p;
}
```

Expected Cust result: `54`.

`tests/fixtures/valid/pointer_parameters.c`

```c
int inc(int *p) {
    *p = *p + 1;
    return *p;
}

int main() {
    int x = 6;
    int y = inc(&x);
    return x * 10 + y;
}
```

Expected Cust result: `77`.

`tests/fixtures/valid/pointer_arrays.c`

```c
int sum3(int *values) {
    values[1] = values[1] + 5;
    return values[0] + values[1] + values[2];
}

int main() {
    int values[3];
    values[0] = 1;
    values[1] = 2;
    values[2] = 3;
    return sum3(values);
}
```

Expected Cust result: `11`.

`tests/fixtures/valid/pointer_string_reads.c`

```c
int second(char *text) {
    return text[1];
}

int main() {
    return second("abc");
}
```

Expected Cust result: `98`.

### Invalid fixtures

`tests/fixtures/invalid/null_pointer_dereference.c`

```c
int main() {
    int *p = 0;
    return *p;
}
```

Expected diagnostic contains `null pointer dereference`.

`tests/fixtures/invalid/pointer_out_of_scope.c`

```c
int main() {
    int *p = 0;
    {
        int x = 7;
        p = &x;
    }
    return *p;
}
```

Expected diagnostic contains `pointer to out-of-scope variable 'x'`.

`tests/fixtures/invalid/pointer_array_out_of_bounds.c`

```c
int main() {
    int values[2];
    int *p = values;
    return p[2];
}
```

Expected diagnostic contains `array pointer index 2 out of bounds for length 2`.

`tests/fixtures/invalid/pointer_string_write.c`

```c
int overwrite(char *text) {
    text[0] = 'x';
    return 0;
}

int main() {
    return overwrite("abc");
}
```

Expected diagnostic contains `cannot modify read-only array`.

## Recommended next work package

The scalar pointer, pointer parameter/decay, pointer indexing, and array-element pointer milestones are implemented. Next, move beyond pointer-specific roadmap work into deterministic fuzz/property tests for lexer/parser safety, then continue conformance fixtures and product-quality CLI flags as prioritized in `status/todo.md`.
