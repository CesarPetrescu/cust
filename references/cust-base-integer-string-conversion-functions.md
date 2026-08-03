# Bounded base-aware integer string conversions

## Scope

Cust supports explicitly prototyped `strtol`, `strtoll`, `strtoul`, and `strtoull` without delegating to host libc. Supported integer spellings normalize to Cust's deterministic 64-bit scalar model. The intrinsic requires the narrow unqualified `char **` output-parameter model established by the preceding safe two-level character-pointer slice.

## Runtime model

- Evaluate input and base once in Cust's established source-order intrinsic model; resolve the restricted `endptr` slot without host addresses.
- Accept base `0` or `2..=36`; base zero selects hexadecimal for a valid `0x` prefix, octal for a leading zero, and decimal otherwise. Explicit base 16 accepts a valid `0x` prefix.
- Skip C whitespace, accept one optional sign, consume the maximal valid digit prefix case-insensitively, return zero with the original input pointer when no digit converts, and write the first unrecognized character for partial conversions.
- Bound the complete NUL-terminated input sequence at 4,096 bytes and diagnose deterministic signed/unsigned magnitude overflow rather than exposing host `errno` or width.
- Preserve input storage identity, lexical owner, lifetime, and read-only metadata in a non-null `endptr` result. Read-only string literals are valid persistent results; mutable ownerless array compound literals are rejected when their end pointer would escape through a non-null output slot.
- Enum constants that visibly resolve to zero are accepted as null `endptr`; the visibility check must precede outer enum lookup so a shadowing object cannot bypass slot validation.

## Non-evaluating validation

`sizeof(call)` validates declaration, arity, input pointer shape, `endptr` shape, and the complete base-expression metadata without evaluating operands. The base-expression helper owns its recursive validation; the surrounding intrinsic walker revisits only the first pointer argument. This prevents duplicate nested traversal while still diagnosing invalid calls hidden in comma-left pointer expressions.

## TDD and review closure

The inherited feature tests and implementation were preserved and verified. Independent review then produced three focused RED/GREEN regressions:

1. an escaped mutable array compound-literal end pointer initially remained usable after its block ended;
2. `sizeof(strtol("7", 0, missing))` initially accepted an undefined base identifier;
3. a visible enum constant with value zero was initially rejected as a null `endptr`.

Reject ownerless mutable retained storage, run metadata-only `sizeof_expr` validation on the base, keep nested traversal single-owner, and admit visible enum-zero null constants. Re-run the shared `base_integer_string` filter, the actual compiler-oracle harness function, recursion-depth coverage, independent re-review, and the complete canonical local/Docker gate after the final edit.

## Native fixture guidance

Use standard relationships only: warning-free parsing results, `endptr` offsets/characters, unsigned cast relationships, and ABI-independent `sizeof(call) == sizeof(return type)`. Do not compare host overflow/`errno`, host integer widths, or host unsigned ordering against Cust's deterministic scalar model.
