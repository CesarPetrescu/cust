# Ordinary tracked unqualified `char **` objects

## Scope

Cust generalizes its narrow `char **` output-parameter metadata to ordinary local, file-global, and block-static objects while retaining interpreter-owned identities rather than host addresses.

Supported objects:

- use the direct unqualified `char **` spelling;
- default or initialize to null, or initialize from `&slot` where `slot` is a mutable unqualified `char *` object;
- support `*output` reads and null/non-null writes;
- forward through existing unqualified `char **` parameters;
- forward conditional/comma object values while preserving the selected outer slot and one-time evaluation;
- preserve outer-slot and pointee owner, lifetime, type, and read-only metadata;
- support equality, truthiness, and non-evaluating `sizeof`.

The safe boundary rejects qualified target slots, qualified/deeper/array `char **` declarators, incompatible and const pointees, expired slots or returned pointees, outer-object reassignment/increment/address-taking, and arithmetic.

## Representation

`CharacterPointerOutput` remains metadata-only:

- `Null`, or
- `Slot { scope_id, name }`.

Ordinary scopes store object metadata in `Scope::character_pointer_outputs`; static locals store it in `StaticLocalStorage::character_pointer_output`. A small scalar placeholder keeps the existing variable table exhaustive, but every pointer-like operation must consult the character-output metadata before scalar fallback. Never turn the placeholder into an address.

## Parser checklist

1. Detect the second star only for direct scalar `char *` declarations.
2. Reject qualifiers at the base, first-star, or second-star boundary with a source-located diagnostic.
3. Lower the declarator to `Stmt::CharacterPointerOutputDecl`.
4. Preserve comma declaration-list behavior.
5. Reject a third star and pointer-array suffix before generic declarator diagnostics.
6. Record whether ordinary `PointerDecl` slots carry any qualifier, including `volatile`, `restrict`, `_Atomic`, or qualifier-bearing typedef metadata. Const-only runtime metadata is not sufficient for unqualified `char **` compatibility.

## Runtime checklist

1. Resolve `&slot` to a `CharacterPointerOutput::Slot` only when the slot is a mutable, unqualified `char *` object.
2. Preserve the outer slot's lexical scope identity, including static-local storage identities.
3. Re-resolve and validate the outer slot on every indirect read/write or forwarding operation.
4. Preserve the written `char *` pointer's owner and read-only metadata exactly.
5. Check pointee owner liveness before later dereference.
6. Check character-output metadata before generic scalar evaluation, truthiness, equality, `sizeof`, assignment, compound assignment, increment, address-of, and arithmetic.
7. Do not permit host-address conversion or general deeper-pointer behavior.

## Non-evaluating validation pitfall

`pointer_expr_pointee_type()` returns the first available pointer type for a conditional expression and does not prove that both branches are compatible. Character-output assignment validation must recurse through both conditional branches before using that classifier. Otherwise:

```c
sizeof(*output = (flag ? char_pointer : int_pointer))
```

can incorrectly depend on branch order while evaluating neither branch.

Null literal and visible enum-zero constants remain accepted null pointer constants; object shadowing must prevent an outer enum constant from being used accidentally.

## Conditional/comma expression parity

- Runtime conditional selection evaluates the condition once and only the selected output-valued branch; comma expressions evaluate the discarded left operand once before returning the right output identity.
- Classify output-valued conditionals before generic pointer inference in `sizeof`, because ordinary `char **` objects deliberately use scalar placeholders in the value table.
- Validate both conditional branches as output values or null. A non-null scalar branch must report `conditional character pointer output branches require compatible output values or null` even though no branch is evaluated.
- Discarded comma operands remain constraint-checked. In particular, a void user-function call with the wrong arity must retain its exact call diagnostic beneath `sizeof`.
- Keep nested validation linear: when a scalar condition is itself output-valued, recurse directly through character-output metadata before whole-subtree void/scalar walkers. The depth-8/depth-40 regression protects this ordering.
- Equality and truthiness are supported for selected output values; relational ordering remains the exact `character pointer output ordering comparisons are not supported` boundary.
- A function parameter may receive the selected/comma-produced identity only when every non-null path names a mutable unqualified `char *` slot. Qualified slot addresses remain rejected.

## TDD and review closure

Focused RED/GREEN regressions covered:

- `volatile`, `restrict`, `_Atomic`, and qualifier-bearing typedef target slots;
- incompatible conditional assignment branches in both orders beneath `sizeof`;
- conditional/comma forwarding, selected-slot equality/truthiness, and qualified parameter-slot rejection;
- direct conditional-output `sizeof`, invalid non-null scalar branches, discarded ordinary-call arity, and linear nested validation;
- third-star and pointer-array declarators;
- direct, compound, prefix/postfix, and non-evaluating outer-object reassignment attempts.

The warning-free native fixture is `tests/fixtures/compat/valid/character_pointer_objects.c` and is registered explicitly in `tests/c_compat.rs`. Run the actual compiler-oracle harness, not a fixture-name filter:

```bash
cargo test --test interpreter character_pointer_object -- --nocapture
cargo test --test c_compat -- --nocapture
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
```

Then obtain independent review and run the canonical local/Docker gate after the final code/test edit.
