# Cust pointer-typedef scalar output objects and parameters

## Scope

Cust's tracked scalar output representation supports interpreter-owned `char **`, `int **`, `_Bool **`, and `double **` objects and parameters. The parser previously admitted only explicit two-star spellings. Equivalent declarations whose inner pointer came from a typedef, such as `typedef int *IntPtr; IntPtr *output`, stopped at the generic pointer-to-pointer rejection before they could reuse that representation.

## Parser pattern

The declaration base may be either:

- `DeclType::Scalar(pointee)` followed by two explicit stars; or
- `DeclType::Pointer { pointee: PointeeType::Scalar(pointee), .. }` followed by one explicit outer star.

Classify both forms as the same tracked scalar output kind. Consume a second star only for the direct scalar spelling. Apply this distinction consistently in function-parameter parsing, first object declarators, and later comma-separated declarators. Alias-backed parameters also need the ordinary declarator-name branch after the outer star; the old pointer-alias branch was unreachable because every explicit star had been rejected earlier.

## Qualification and boundary rules

Do not infer mutability from the flattened scalar pointee alone. Reject output forms when any of these qualify the supported unqualified tracked shape:

- the pointer alias carries pointee qualification (`typedef const int *View` or `volatile`/`_Atomic` equivalents);
- the pointer alias carries pointer-slot qualification (`typedef int * const Slot`);
- the explicit outer pointer star is qualified (`ValuePtr * const output`); or
- a direct explicit inner/outer star is qualified.

Keep non-scalar pointer aliases outside this exception. Aggregate pointer aliases followed by `*`, extra stars, pointer arrays, aggregate fields, and pointer-return forms retain their existing exact unsupported diagnostics. Chained pointer aliases must preserve both the scalar pointee and qualification metadata.

## Verification shape

A focused generated interpreter test should cross all four scalar pointees with local, file-global, and block-static output objects; direct and later comma declarators; parameter forwarding; chained aliases; indirect writes; and non-evaluating `sizeof`. Add exact qualification, lifetime, deeper-pointer, array, aggregate-field, aggregate-alias, and return-boundary checks.

Register a warning-free C11 fixture in `tests/c_compat.rs`. It should compare only portable pointer relationships and same-type `sizeof` relationships, include direct-versus-typedef prototype compatibility, and avoid host pointer representation assumptions. Native compilers remain test oracles only; Cust continues to use interpreter-owned tracked identities.
