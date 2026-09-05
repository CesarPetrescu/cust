# Cust pointer-typedef scalar output objects and parameters

## Scope

Cust's tracked scalar output representation supports interpreter-owned `char **`, `int **`, `_Bool **`, and `double **` objects and parameters. Parser closure happened in two stages: inner pointer aliases such as `typedef int *IntPtr; IntPtr *output`, then aliases of the complete tracked shape such as `typedef IntPtr *IntOutput; IntOutput output`. Both reuse the same runtime representation.

## Parser pattern

The declaration base may be either:

- `DeclType::Scalar(pointee)` followed by two explicit stars; or
- `DeclType::Pointer { pointee: PointeeType::Scalar(pointee), .. }` followed by one explicit outer star; or
- `DeclType::PointerOutput(pointee)` followed by no explicit star.

Classify all forms as the same tracked scalar output kind. Consume a second star only for the direct scalar spelling and consume no star for a complete output alias at its use site. Apply this distinction consistently in function-parameter parsing, first object declarators, and later comma-separated declarators. Represent a complete alias explicitly as `TypeAlias::PointerOutput(CType)` / `DeclType::PointerOutput(CType)` so chained aliases preserve the scalar pointee and unsupported consumers can retain exact diagnostics instead of losing a pointer level.

## Qualification and boundary rules

Do not infer mutability from the flattened scalar pointee alone. Reject output forms when any of these qualify the supported unqualified tracked shape:

- the pointer alias carries pointee qualification (`typedef const int *View` or `volatile`/`_Atomic` equivalents);
- the pointer alias carries pointer-slot qualification (`typedef int * const Slot`);
- the explicit outer pointer star is qualified (`ValuePtr * const output`); or
- a direct explicit inner/outer star is qualified; or
- a complete output alias carries top-level qualification (`typedef ValuePtr * const Output`).

Keep non-scalar pointer aliases outside this exception. Aggregate/void pointer aliases followed by `*`, extra stars, pointer arrays, aggregate fields, casts, `_Atomic` wrappers, and pointer-return forms retain their existing exact unsupported diagnostics. An explicit star after `DeclType::PointerOutput` is a genuine third pointer level. Chained pointer and output aliases must preserve both the scalar pointee and qualification metadata.

## Verification shape

A focused interpreter suite should cross all four scalar pointees with local, file-global, and block-static output objects; direct and later comma declarators; parameter forwarding; chained inner and complete aliases; indirect writes; and non-evaluating `sizeof`. Add exact qualification, lifetime, static-storage, `_Atomic`, deeper-pointer, array, aggregate-field, aggregate-alias, cast, and return-boundary checks. When retaining a deeper-pointer diagnostic, ensure the source constructs a third level after complete two-level aliases became supported.

Register a warning-free C11 fixture in `tests/c_compat.rs`. It should compare only portable pointer relationships and same-type `sizeof` relationships, include direct-versus-typedef prototype compatibility, and avoid host pointer representation assumptions. Native compilers remain test oracles only; Cust continues to use interpreter-owned tracked identities.
