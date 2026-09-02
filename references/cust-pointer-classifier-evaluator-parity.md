# Cust Pointer Classifier/Evaluator Parity

## Scope

Use this note when extending pointer-valued `Expr` routes, pointer conversion checks, or generated classifier/evaluator property coverage.

The 2026-09-02 closure models 69 stable pointer route identities across eight consumers:

- compatible initializer
- compatible function argument
- compatible function return
- non-evaluating `sizeof`
- incompatible initializer
- incompatible function argument
- incompatible function return
- incompatible assignment

That is 552 exact generated programs. Keep route, context, and cell totals keyed by stable enum identity, not iteration position. Separate const-preservation cases currently cover seven structural wrapper families.

## Metadata parity checklist

For every pointer-valued route admitted by `expr_is_pointer_value()` audit all of:

1. `pointer_expr_pointee_type()`
2. `pointer_expr_points_to_const()`
3. `eval_pointer()`
4. `sizeof_expr()` / non-evaluating generic-selection typing
5. initializer, argument, return, statement-assignment, and assignment-expression conversion sites
6. lexical owner/lifetime propagation
7. compatible and incompatible conversion diagnostics

One AST variant can represent multiple storage shapes. `Expr::AddressOfArray`, for example, reaches scalar arrays, aggregate arrays, and pointer indexing. Enumerating the AST variant once is not enough unless each storage shape's pointee and const metadata is covered.

## Pre-evaluation conversion rule

C pointer type constraints are semantic checks. An incompatible conversion must be rejected before evaluating any operand side effects. A regression such as:

```c
int values[1] = {1};
char chars[1] = {2};
int trap(void) { return 1 / 0; }
int *pointer = values;
/* Must diagnose char * -> int * without calling trap(). */
pointer = (trap(), chars);
```

must produce the exact incompatible-pointer diagnostic, not `division by zero`.

Apply metadata-only validation before `eval_pointer()` in all matching paths:

- automatic and static-local pointer declarations
- function argument/return binding
- pointer statement assignments
- pointer-valued assignment expressions
- direct, aggregate-literal, struct-pointer, aggregate-array-element, and embedded aggregate-array-element pointer fields

Do not infer this closure from outer incompatible consumers alone. The generated route expression can itself contain an assignment whose RHS is incompatible; add dedicated nested-RHS regressions.

## Compound-literal lifetime

C block-scope compound literals have enclosing-block storage duration. Hidden scalar-array compound-literal roots therefore need lexical owners for every scalar element type, not only `double`. Same-block library calls may retain/use the pointer during that scope; escaped pointers must later report the hidden root as out of scope.

## Review lesson

Count-only GREEN can hide a missing storage-shape cell. Independent review should probe:

- aggregate-array element addresses (`&items[i]`)
- const aggregate-array elements
- nested incompatible assignment RHS expressions
- static-local pointer initializers
- two-dimensional row conversions, including the supported qualification-preserving `void *` boundary
