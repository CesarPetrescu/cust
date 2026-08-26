# Changelog

All notable changes to Cust are documented here. Cust is still a small educational C-subset interpreter; release notes describe the supported subset, diagnostics, tooling, and verification status at each tag.

## Unreleased

### Language subset

- Added direct and typedef-backed one-dimensional `double` array compound literals with fixed or inferred lengths, positional and designated initialization, zero fill, hidden-root pointer decay/index/update, const preservation, lexical lifetime checks, one-time evaluated initialization, and non-evaluating full-object/element type queries.

### Diagnostics and verification

- Retained exact multidimensional, whole-array-address/pointer-to-row, union-backed double-field decay, and raw-memory double-storage boundaries in evaluated and non-evaluating contexts. Focused direct/typedef valid and invalid fixtures cover const discard/write, expired hidden storage, deterministic size/alignment relationships, and warning-free GCC/Clang/compiler-oracle parity.

## v0.36.0 — 2026-08-25

### Language subset

- Added typedef aliases for supported direct `double` forms. Scalar and const values, fixed one-dimensional arrays and adjusted parameters, direct function parameters/returns, supported aggregate scalar/array/pointer fields, and one-level pointers preserve double type, qualification, owner, lifetime, bounds, and non-evaluating metadata.

### Diagnostics and verification

- Retained targeted deeper-pointer, array-of-pointer, pointer-to-row, multidimensional, union-backed-pointer, raw-memory, atomic-pointer-alias, and double-array-compound-literal boundaries. Review-driven RED/GREEN closed six alias-derived declarator and pointer-slot-qualification bypasses. Three focused interpreter tests and one registered warning-free compiler-oracle fixture cover the package.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.36.0`.
- Reconciled an executable inventory of 2,060 tests: 1,924 interpreter tests, 99 deterministic fuzz-safety tests, 33 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- The bounded `double` slice supports direct and typedef-backed scalar/function/one-dimensional-array/aggregate-field storage plus one-level pointers to standalone and supported struct-field scalar/one-dimensional-array storage. `float`, `long double`, hexadecimal/suffixed/non-finite floating literals, deeper double pointers, arrays of double pointers, pointer-to-row forms such as `double (*)[N]`, whole-array addresses, union-backed double field addresses/decay, atomic double-pointer aliases, double-array compound literals, multidimensional double arrays, and raw-memory operations over double storage remain unsupported. Complex runtime values, host floating ABI/promotion behavior, general pointer levels beyond the narrow tracked unqualified `char **` model, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.35.0 — 2026-08-24

### Language subset

- Extended safe one-level direct `double *` support to scalar and one-dimensional-array fields in supported structs. Direct, indexed, reverse-subscript, arrow, nested, compound-literal, and aggregate-valued expression routes preserve interpreter-owned root, element, field, const, and lexical-lifetime identity through pointer fields, calls, returns, conditionals, comma expressions, and array decay.

### Diagnostics and verification

- Retained targeted deeper-pointer, pointer-to-row/multidimensional, whole-array-address, raw-memory double-storage, and union-backed double-field address/decay boundaries. Review-driven RED/GREEN also prevents static-local call analysis from laundering union-backed provenance through a concrete safe aggregate target. Focused valid/invalid regressions and a registered warning-free compiler-oracle fixture cover the slice.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.35.0`.
- Reconciled an executable inventory of 2,057 tests: 1,921 interpreter tests, 99 deterministic fuzz-safety tests, 33 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- The bounded `double` slice supports direct scalar/function/one-dimensional-array/aggregate-field storage plus one-level pointers to standalone and supported struct-field scalar/one-dimensional-array storage. `float`, `long double`, hexadecimal/suffixed/non-finite floating literals, double typedef aliases, deeper double pointers, pointer-to-row forms such as `double (*)[N]`, whole-array addresses, union-backed double field addresses/decay, double-array parameters and compound literals, multidimensional double arrays, and raw-memory operations over double storage remain unsupported. Complex runtime values, host floating ABI/promotion behavior, general pointer levels beyond the narrow tracked unqualified `char **` model, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.34.0 — 2026-08-23

### Language subset

- Added a safe first one-level direct `double *` slice over interpreter-owned scalar and one-dimensional-array storage. Local/global/static objects, scalar addresses, array decay/indexing/arithmetic/equality/truthiness, pointer fields, function parameters/returns, qualification-preserving `void *` conversion, and non-evaluating `sizeof`/`_Alignof`/`_Generic` queries preserve Cust owner, lifetime, and const metadata.

### Diagnostics and verification

- Retained exact deeper-pointer, pointer-to-row/multidimensional, aggregate-field-address/decay, typedef-double, and raw-memory double-storage boundaries. Focused valid/invalid regressions and a registered warning-free compiler-oracle fixture cover the slice.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.34.0`.
- Reconciled an executable inventory of 1,977 tests: 1,841 interpreter tests, 99 deterministic fuzz-safety tests, 33 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- The bounded `double` slice supports direct scalar/function/one-dimensional-array/aggregate-field storage plus one-level pointers to standalone scalar and one-dimensional-array storage. `float`, `long double`, hexadecimal/suffixed/non-finite floating literals, double typedef aliases, deeper double pointers, aggregate-field double addresses/decay, double-array parameters and compound literals, multidimensional double arrays, raw-memory operations over double storage, and unions mixing double storage with non-scalar layouts remain unsupported. Complex runtime values, host floating ABI/promotion behavior, general pointer levels beyond the narrow tracked unqualified `char **` model, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.33.0 — 2026-08-17

### Language subset

- Extended direct one-dimensional `double` arrays into supported aggregate fields. Positional/designated initialization and zero fill, direct/indexed/arrow/nested/reverse element reads and replacement/compound/prefix/postfix updates, aggregate pointer-field element routes, aggregate copies and function boundaries, conditional/comma/`_Generic` wrappers, deterministic full-field/element queries, recursive const protection, and one-time index evaluation preserve Cust's deterministic double representation.

### Diagnostics and verification

- Retained the unsupported direct-double-pointer/ordinary-decay, double-array-parameter, double-array-compound-literal, multidimensional-double-array, raw-memory-view, and mixed non-scalar-union boundaries. Review-driven RED/GREEN preserved pointer-field pointee qualification through reverse subscripts and validated const array/nested-aggregate ancestry in non-evaluating mutation routes. Focused interpreter/invalid fixtures and warning-free ABI-independent GCC/Clang compiler-oracle fixtures cover the slice.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.33.0`.
- Reconciled an executable inventory of 1,715 tests: 1,580 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- The bounded `double` slice supports direct scalar storage and function boundaries, direct one-dimensional arrays, and scalar plus one-dimensional-array aggregate fields. `float`, `long double`, hexadecimal/suffixed/non-finite floating literals, double typedef aliases, direct double pointers and ordinary array decay, address-taking or dereference, double-array parameters and compound literals, multidimensional double arrays, raw-memory operations over double storage, and unions mixing double storage with non-scalar layouts remain unsupported. Complex runtime values, host floating ABI/promotion behavior, general pointer levels beyond the narrow tracked unqualified `char **` model, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.32.0 — 2026-08-14

### Language subset

- Extended bounded `double` storage to direct scalar fields in supported structs and unions. Positional/designated initialization, direct/indexed/arrow/nested reads, replacement/compound/prefix/postfix updates, aggregate copies and function boundaries, deterministic field `sizeof`, recursive const protection, and shared scalar-union bits preserve Cust's deterministic double representation.

### Diagnostics and verification

- Retained the unsupported direct-double-pointer, one-dimensional-double-array aggregate-field, multidimensional-double-array, raw-memory-view, and mixed non-scalar-union boundaries. Review-driven RED/GREEN completed aggregate expression routing for nested fields reached through indexed and arrow expressions. Focused interpreter/invalid fixtures and a warning-free ABI-independent GCC/Clang compiler oracle cover the slice.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.32.0`.
- Reconciled an executable inventory of 1,642 tests: 1,507 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- The bounded `double` slice supports direct scalar storage, expressions and function boundaries, direct one-dimensional arrays, and direct scalar aggregate fields. `float`, `long double`, hexadecimal/suffixed/non-finite floating literals, double typedef aliases, double pointers and ordinary array decay, address-taking or dereference, double-array parameters and compound literals, multidimensional or aggregate-field double arrays, raw-memory operations over double storage, and unions mixing double storage with non-scalar layouts remain unsupported. Complex runtime values, host floating ABI/promotion behavior, general pointer levels beyond the narrow tracked unqualified `char **` model, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.31.0 — 2026-08-13

### Language subset

- Extended bounded `double` storage to direct one-dimensional arrays with fixed or initializer-inferred lengths, positional/designated initialization and zero fill, local/file-global/block-static storage, direct and reverse indexed reads, replacement/compound/prefix/postfix updates, integer/double conversion, `_Generic`, and deterministic object/element/type-name `sizeof` plus `_Alignof` relationships.

### Diagnostics and verification

- Preserved exact const and bounds checks while retaining the unsupported double-pointer/decay, double-array-parameter, double-array-compound-literal, and multidimensional-double-array boundaries. Review-driven RED/GREEN closed reverse-subscript result typing and numeric increment, retained bounded raw-memory intrinsic diagnostic precedence, and sharpened source-located parameter/compound-literal diagnostics. Focused interpreter/invalid fixtures and a warning-free ABI-independent GCC/Clang compiler oracle cover the slice.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.31.0`.
- Reconciled an executable inventory of 1,598 tests: 1,463 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- The bounded `double` slice supports direct scalar storage, expressions, function boundaries, and direct one-dimensional arrays. `float`, `long double`, hexadecimal/suffixed/non-finite floating literals, double typedef aliases, aggregate fields, double pointers and ordinary array decay, address-taking or dereference, double-array parameters and compound literals, and multidimensional double arrays remain unsupported. Raw-memory intrinsics still reject double object storage. Complex runtime values, host floating ABI/promotion behavior, general pointer levels beyond the narrow tracked unqualified `char **` model, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.30.0 — 2026-08-12

### Language subset

- Extended bounded `double` values through direct function parameter and return declarations. Prototypes, definitions, named and unnamed parameters, recursive calls, integer/double argument and return conversion, casts, `_Generic`, and non-evaluating `sizeof(call)` preserve exact scalar metadata and one-time evaluation.

### Diagnostics and verification

- Preserved source-located rejection for direct double pointers, arrays, pointer-to-row returns/parameters, and non-`int` `main`. Review-driven regressions close pointer/array declarator bypasses, pointer-to-row bypasses, raw IEEE entry-point results, and a row-pointer `main` declarator bypass. The warning-free ABI-independent native fixture passes GCC, Clang, and Cust's compiler-oracle harness.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.30.0`.
- Reconciled an executable inventory of 1,594 tests: 1,459 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- The `double` slice remains limited to direct scalar storage, scalar expressions, and direct function boundaries. `float`, `long double`, hexadecimal/suffixed/non-finite floating literals, double typedef aliases, arrays, aggregate fields, pointers, and pointer-to-row forms remain unsupported. Complex runtime values, host floating ABI/promotion behavior, general pointer levels beyond the narrow tracked unqualified `char **` model, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.29.0 — 2026-08-12

### Language subset

- Added a bounded first `double` runtime-value slice. Decimal literals and direct local, file-global, and block-static scalar objects support initialization, replacement and compound assignment, prefix/postfix update, mixed arithmetic and comparison, conditional/comma forwarding, truthiness, scalar compound literals, `int`/`double`/`_Bool` casts, `_Generic`, integer-constant casts, conversion through supported scalar function signatures, and deterministic eight-byte size/alignment queries.

### Diagnostics and verification

- Preserved targeted boundaries for `float`, `long double`, unsupported floating literal spellings, double arrays and aggregate fields, double pointers, direct double parameter/return declarations, and invalid floating remainder/bitwise/shift operations. Review-driven regressions cover fractional `_Bool` conversion, assignment/call/return pointer escapes, non-evaluating literal and generic validation, and linear nested generic/call classification. A registered warning-free native fixture covers ABI-independent relationships.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.29.0`.
- Reconciled an executable inventory of 1,592 tests: 1,457 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- The `double` slice is limited to direct scalar storage and expressions. `float`, `long double`, hexadecimal/suffixed/non-finite floating literals, double arrays, aggregate fields, pointers, and direct double parameter/return declarations remain unsupported. Complex runtime values, host floating ABI/promotion behavior, general pointer levels beyond the narrow tracked unqualified `char **` model, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.28.0 — 2026-08-10

### Standard library subset

- Extended exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` to complete supported struct objects under Cust's deterministic field-order, no-padding layout. Full and interior ranges traverse scalar fields, one- and two-dimensional scalar arrays, nested structs, and embedded struct arrays without host addresses or host ABI inference.

### Diagnostics and verification

- Preserved canonical aligned/interior pointer identity, selected-range capacity and overlap, owner and lexical-lifetime identity, recursive const protection, zero-count behavior, and nested non-evaluating validation. Union-backed subobjects and pointer fields remain exact unsupported boundaries because Cust does not model shared union bytes or pointer object encodings. Fifty-six focused whole-struct tests and one registered ABI-independent compiler-oracle fixture are included.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.28.0`.
- Reconciled an executable inventory of 1,534 tests: 1,399 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Raw-memory intrinsics accept interpreter-owned character and deterministic scalar storage, selected two-dimensional rows, supported struct fields, and complete supported structs under Cust's field-order no-padding model. Union-backed storage, structs containing pointer fields, host ABI padding/layout, and host addresses remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, aggregate-valued multidimensional elements, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.27.0 — 2026-08-09

### Standard library subset

- Extended exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` to one selected row of supported two-dimensional `int` and `_Bool` arrays. Direct row decay, adjusted parameters, explicit pointer-to-row indexing/dereference, and supported struct-field rows retain interpreter-owned storage and dimensions without flattening into adjacent rows.

### Diagnostics and verification

- Preserved row-local byte capacity and overlap, owner and lexical-lifetime identity, recursive const protection, checked row-index overflow, aligned/interior `memchr` identity, zero-count behavior, and nested non-evaluating validation. Whole aggregates and union-backed storage remain exact unsupported boundaries. Four focused interpreter tests and one registered ABI-independent compiler-oracle fixture bring the executable inventory to 1,479 tests.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.27.0`.
- Reconciled an executable inventory of 1,479 tests: 1,344 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Raw-memory intrinsics accept interpreter-owned standalone character storage, character scalars or one-dimensional character arrays embedded in struct fields, one selected row of supported two-dimensional character storage, and scalar/one-dimensional-array/selected-two-dimensional-row `int` and `_Bool` storage both standalone and in supported struct fields. Every range remains selected-object, field, or row local. Whole aggregate object representations, union-backed storage, host ABI layout, and host addresses remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, aggregate-valued multidimensional elements, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.26.0 — 2026-08-09

### Standard library subset

- Extended exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` to deterministic `int`/`_Bool` scalar and one-dimensional-array fields in supported named, anonymous, nested, direct/arrow, and embedded-aggregate-array-element struct paths. The implementation reuses the selected field's interpreter-owned storage rather than synthesizing aggregate padding or host addresses.

### Diagnostics and verification

- Preserved field-local byte capacity and overlap, recursive const and lexical-lifetime checks, aligned/interior `memchr` identity, canonical typed-pointer coercion, `_Bool` normalization, and non-evaluating call validation. Whole aggregates, union-backed storage, and two-dimensional non-character rows retain exact unsupported diagnostics. Five focused interpreter tests and a registered warning-free ABI-independent compiler-oracle fixture bring the executable inventory to 1,475 tests.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.26.0`.
- Reconciled an executable inventory of 1,475 tests: 1,340 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Raw-memory intrinsics accept interpreter-owned standalone character storage, character scalars or one-dimensional character arrays embedded in struct fields, one selected row of supported two-dimensional character storage, and scalar/one-dimensional-array `int` and `_Bool` storage both standalone and embedded in supported struct fields. Ranges remain selected-object or field local and cannot cross character-row boundaries. Two-dimensional non-character rows, whole aggregate object representations, union-backed storage, host ABI layout, and host addresses remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, aggregate-valued multidimensional elements, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.25.0 — 2026-08-08

### Standard library subset

- Extended exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` to standalone `int`/`_Bool` scalars and one-dimensional arrays under Cust's fixed little-endian eight-byte integer representation. Partial/full reads and writes, overlap-safe snapshots, unsigned-byte comparison/search, canonical interior pointers, and pointer differences remain interpreter-owned without host addresses or host ABI inference.

### Diagnostics and verification

- Added exact scalar-object capacity, const, lifetime, overlap, aggregate, and union boundaries; deterministic partial-write reconstruction; review-driven aligned `memchr` byte-view/coercion regressions; seven interpreter tests; and a warning-free compiler-oracle fixture limited to ABI-independent relationships.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.25.0`.
- Reconciled an executable inventory of 1,470 tests: 1,335 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Raw-memory intrinsics accept interpreter-owned standalone character storage, character scalars or one-dimensional character arrays embedded in struct fields, one selected row of supported two-dimensional character storage, and standalone scalar/one-dimensional-array `int` and `_Bool` storage. A range cannot cross a row boundary. Non-character aggregate fields, two-dimensional non-character rows, whole aggregate object representations, union-backed storage, host ABI layout, and host addresses remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, aggregate-valued multidimensional elements, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.24.0 — 2026-08-08

### Standard library subset

- Extended exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` to scalar character-pointer views of supported two-dimensional `char` arrays. Direct rows, adjusted parameters, explicit pointer-to-row expressions, and aggregate-field rows preserve interpreter-owned owner/lifetime/const metadata and returned pointer identity without exposing host addresses or flattening into adjacent rows.

### Diagnostics and verification

- Added row-local source/destination capacity and overlap enforcement, interior offsets, zero-count and nested non-evaluating coverage, exact escaped-lifetime and const diagnostics, checked huge-row arithmetic, and const-ancestor propagation after embedded aggregate-array elements. Focused regressions and a registered GCC/Clang-warning-free compiler oracle pass; the executable inventory is 1,463 tests.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.24.0`.
- Reconciled an executable inventory of 1,463 tests: 1,328 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Raw-memory intrinsics accept only interpreter-owned standalone character storage, character scalars or one-dimensional character arrays embedded in struct fields, and one selected row of supported two-dimensional character storage. A range cannot cross a row boundary. Integer and whole-aggregate object representations, union-backed character fields, host ABI layout, and host addresses remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, aggregate-valued multidimensional elements, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.23.0 — 2026-08-08

### Standard library subset

- Extended exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` to character scalars and one-dimensional character arrays embedded in named, anonymous, and nested struct fields. Direct and arrow paths, interior offsets, returned pointer identity, overlap-safe snapshots, unsigned-byte comparison/search, and byte-normalized fills reuse interpreter-owned storage without host addresses or invented aggregate padding.

### Diagnostics and verification

- Preserved field-local capacity, overlap, const, lifetime, zero-count, and non-evaluating validation. Whole aggregate values and non-character fields remain rejected; union-backed character storage now has a dedicated exact boundary because Cust does not yet model raw-memory aliasing between union members. Focused regressions cover direct and embedded-aggregate-array scalar fields, nested array fields, exact unsafe diagnostics, and a registered GCC/Clang-warning-free compiler oracle.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.23.0`.
- Reconciled an executable inventory of 1,460 tests: 1,325 interpreter tests, 99 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Raw-memory intrinsics accept only interpreter-owned standalone character storage and character scalars or one-dimensional character arrays embedded in struct fields. Integer and whole-aggregate object representations, union-backed character fields, multidimensional character arrays, host ABI layout, and host addresses remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.22.0 — 2026-08-07

### Standard library subset

- Added exactly prototyped bounded `memchr` over interpreter-owned standalone character scalars and one-dimensional character arrays. Source, search value, and count evaluate once in source order; stored cells and the search value compare as `unsigned char`; the first matching interior pointer preserves source owner/lifetime/read-only identity; and no match returns null.

### Diagnostics and verification

- Added exact prototype, arity, pointer/value/count-shape, lifetime, capacity, non-character-storage, const-result, and direct/nested non-evaluating `sizeof` validation. User-defined `memchr` bodies retain precedence, including after a matching prototype; focused review-driven regressions cover malformed arity, source-order evaluation, escaped returned pointers, and intrinsic-vs-user const provenance, while a warning-free native fixture covers defined behavior.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.22.0`.
- Reconciled an executable inventory of 1,447 tests: 1,313 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Raw-memory intrinsics accept only interpreter-owned standalone `char` scalars and one-dimensional `char` arrays; integer and aggregate object representations, aggregate-backed character fields, multidimensional character arrays, host ABI layout, and host addresses remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.21.0 — 2026-08-06

### Standard library subset

- Added exactly prototyped bounded `memcpy`, overlap-safe `memmove`, unsigned-byte lexical `memcmp`, and `unsigned char`-normalized `memset` over interpreter-owned standalone character scalars and one-dimensional character arrays. Copy operations snapshot exact Cust character-cell values before mutation and preserve destination identity/provenance; comparison reads up to 4,096 cells without mutation and returns the sign of the first differing byte; fill writes the low eight bits of its scalar value and preserves destination identity. None exposes host addresses or invokes host libc.

### Diagnostics and verification

- Added exact prototype, arity, pointer/value/count-shape, const, lifetime, source/destination/input-capacity, non-character-storage, and non-evaluating `sizeof` validation. `memcpy` rejects overlapping nonzero ranges, `memmove` implements temporary-array semantics for forward/backward/self overlap, `memcmp` permits overlap because it only reads, and `memset` retains the shared 4,096-cell bound. Registered warning-free native fixtures cover defined behavior.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.21.0`.
- Reconciled an executable inventory of 1,437 tests: 1,303 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Raw-memory intrinsics accept only interpreter-owned standalone `char` scalars and one-dimensional `char` arrays; integer and aggregate object representations, aggregate-backed character fields, multidimensional character arrays, host ABI layout, and host addresses remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, and other pragma semantics also remain outside the release.

## v0.20.0 — 2026-08-06

### Language subset

- Added bounded one-level `void *` function return types, prototypes, definitions, and call results. Declared void-pointee qualification and interpreter-owned owner/lifetime/read-only identity survive evaluated return chains, assignments, casts, conditionals, comma expressions, equality/truthiness, and `_Generic` selection.

### Diagnostics and verification

- Preserved exact pointer-to-pointer return and unsafe void-pointer operation diagnostics, and made prototype-only plus defined calls beneath `sizeof` constraint-aware without argument evaluation. Review-driven regressions cover const-qualified two-dimensional row conversion and defined-function argument validation. The registered warning-free compiler-oracle fixture and 1,400-test executable inventory cover the completed slice.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.20.0`.
- Verified an executable inventory of 1,400 tests: 1,266 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- `void *` dereference, indexing, arithmetic, ordering, deeper pointers, pointer arrays, pointer-to-array forms, function pointers, and raw-memory intrinsics remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, other pragma semantics, and host ABI layout/promotion rules also remain outside the release.

## v0.19.0 — 2026-08-05

### Language subset

- Added bounded one-level C11 `void *` objects at local, file-global, block-static, `for`-initializer, and parameter scope. Null/default state, equality, truthiness, ordinary assignment, conditional/comma forwarding, and qualification-preserving compatible conversions to and from supported object pointers retain interpreter-owned owner, lexical-lifetime, and read-only identity without exposing host addresses.

### Diagnostics and verification

- Preserved exact diagnostics for dereference, indexing, arithmetic, relational ordering, qualification discard, incompatible conversion, deeper pointers, pointer arrays, pointer-to-array forms, and function-pointer forms, including structural validation beneath non-evaluating `sizeof`. Focused valid/invalid interpreter fixtures and a registered warning-free compiler-oracle fixture cover the bounded surface.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.19.0`.
- Verified an executable inventory of 1,394 tests: 1,260 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- `void *` function return/prototype/call-result forms, dereference, indexing, arithmetic, ordering, deeper pointers, pointer arrays, pointer-to-array forms, and memory intrinsics remain unsupported. General pointer levels beyond the narrow tracked unqualified `char **` model, function pointers, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, other pragma semantics, and host ABI layout/promotion rules also remain outside the release.

## v0.18.0 — 2026-08-05

### Language subset

- Added bounded C11 `_Generic` selection over deterministic scalar, one-level pointer, and named aggregate association types. The controlling expression is type-classified without evaluation, exactly one compatible association or optional `default` is selected, and only the selected expression executes while preserving scalar, pointer, aggregate, discard, `sizeof`, and integer-constant-expression behavior.

### Diagnostics and verification

- Added exact source-located diagnostics for duplicate compatible associations, duplicate defaults, unsupported `void`/array/function/anonymous-aggregate/deeper-pointer association forms, and no-match selections without a default. Focused interpreter and invalid fixtures plus a registered warning-free compiler-oracle fixture cover the bounded surface.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.18.0`.
- Verified an executable inventory of 1,389 tests: 1,255 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- `_Generic` associations remain bounded to deterministic scalar, one-level pointer, and named aggregate types; two-dimensional array controlling expressions remain unsupported. General `void *`, function pointers, deeper pointer levels beyond the narrow tracked unqualified `char **` model, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.17.0 — 2026-08-04

### Language subset

- Added ordinary reassignment for narrow tracked unqualified `char **` objects. Local, file-global, block-static, and parameter objects may select null, a compatible mutable unqualified `char *` slot address, or a compatible tracked object value while preserving outer-slot and pointee owner/lifetime/read-only identity.
- Assignment results forward through initializers, equality, truthiness, indirect reads/writes, and `_Bool` conversion with one-time source-order evaluation; parameter-slot reassignment remains local to the callee.

### Diagnostics and verification

- Structural non-evaluating validation accepts compatible assignment forms without mutation or stale-target observation. Exact boundaries remain for incompatible or qualified slots, expired selected owners, deeper pointers, arrays, address-taking, compound updates, increment/decrement, and arithmetic.
- Review-driven regressions cover comma-side-effect repair before liveness checks, non-evaluating `strtol` `endptr` assignment, conditional indirect-assignment targets, nested scalar wrappers, and null/non-null assignment-result `_Bool` normalization.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.17.0`.
- Verified an executable inventory of 1,358 tests: 1,224 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- General multiple-pointer-level objects beyond the narrow tracked unqualified `char **` model, qualified/deeper pointer-to-pointer forms, `char **` address-taking/compound updates/arithmetic, `void *`, function pointers and variadic calls, floating-point and complex runtime values, variable-length arrays, arrays with more than two dimensions, flexible array members, bit-fields, `goto`, system headers, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.16.0 — 2026-08-04

### Language subset

- Completed expression parity for narrow tracked unqualified `char **` objects. Conditional and comma expressions preserve the selected outer-slot identity through parameter forwarding, equality, truthiness, and scalar control contexts with one-time source-order evaluation.

### Diagnostics and verification

- Added exact qualified function-parameter slot-address rejection and relational-ordering boundaries. Non-evaluating `sizeof` validates both conditional branches, discarded comma operands, ordinary call constraints, and deeply nested output-valued conditions without evaluating operands or repeating subtree work.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.16.0`.
- Verified 1,343 executable tests at release preparation time: 1,209 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string/character/random/termination families, locale-sensitive behavior outside Cust's fixed ASCII/C-locale model, floating-point and complex runtime values, general multiple-pointer-level objects beyond the narrow tracked unqualified `char **` model, `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported. Tracked `char **` object reassignment, address-taking, compound updates, and arithmetic remain outside the released slice.

## v0.15.0 — 2026-08-04

### Language subset

- Extended the narrow safe unqualified `char **` model from function output parameters to ordinary local, file-global, and block-static objects. Objects may default or initialize to null or bind to a mutable unqualified `char *` slot, then support indirect null/non-null reads and writes plus forwarding through existing `char **` parameters.

### Diagnostics and verification

- Preserved interpreter-owned outer-slot and pointee owner, lifetime, type, qualification, and read-only identity without host addresses. Exact diagnostics retain the bounded subset around qualified or incompatible target slots, const/deeper/array declarators, expired storage, outer-object reassignment/increment/address-taking, arithmetic, and incompatible conditional assignment branches under non-evaluating `sizeof`.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.15.0`.
- Verified 1,333 executable tests at release preparation time: 1,199 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string/character/random/termination families, locale-sensitive behavior outside Cust's fixed ASCII/C-locale model, floating-point and complex runtime values, general multiple-pointer-level objects beyond the narrow tracked unqualified `char **` model, `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported. Cust reports deterministic conversion overflow rather than modeling host-width `errno` behavior.

## v0.14.0 — 2026-08-03

### Language subset

- Added a narrow safe unqualified `char **` output-parameter model over interpreter-owned mutable `char *` slots. Calls can forward output slots, read or write null/non-null character pointers through them, and preserve lexical owner, lifetime, storage identity, and read-only metadata without host addresses.
- Added exact-prototype C11 `strtol`, `strtoll`, `strtoul`, and `strtoull` intrinsics with base 0 or 2 through 36 selection, C whitespace/sign/prefix/maximal-digit behavior, deterministic signed/unsigned 64-bit bounds, 4,096-byte scans, and absent, null, or non-null `endptr` handling.

### Diagnostics and verification

- Added exact qualified/deeper-pointer, incompatible/const output slot, outer-slot mutation/address-taking, enum-shadowing, expired owner, ownerless escaping storage, base, declaration, arity, shape, overflow, and nested non-evaluating diagnostics. Review-driven regressions preserve one-time source-order evaluation and linear validation.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.14.0`.
- Verified 1,310 tests at release preparation time: 1,176 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string/character/random/termination families, locale-sensitive behavior outside Cust's fixed ASCII/C-locale model, floating-point and complex runtime values, general multiple-pointer-level objects beyond the narrow `char **` output-parameter model, `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported. Cust reports deterministic conversion overflow rather than modeling host-width `errno` behavior.

## v0.13.0 — 2026-08-02

### Language subset

- Added exact-prototype C11 `strcoll` and `strxfrm` intrinsics under Cust's deterministic C-locale model. `strcoll` performs normalized unsigned-byte ordering; `strxfrm` returns the complete transformed length, writes at most the requested count, supports zero-count null-destination queries, and never delegates to host locale or libc.

### Diagnostics and verification

- Added exact declaration, user-definition precedence, arity, pointer/count shape, null, const, owner, lifetime, two-dimensional-row, destination-capacity, overlap, and 4,096-byte bound coverage. Runtime arguments evaluate once in source order, direct and nested `sizeof` remain non-evaluating but constraint-aware, and a review-driven depth regression keeps nested validation linear.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.13.0`.
- Verified 1,279 tests at release preparation time: 1,145 interpreter tests, 98 deterministic fuzz-safety tests, 32 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string/character/random/termination families, locale-sensitive behavior outside Cust's fixed ASCII/C-locale model, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported. `strtol` and `strtoul` remain deferred until safe pointer-to-pointer `endptr` semantics exist.

## v0.12.0 — 2026-08-02

### Language subset

- Added exact-prototype C11 `exit(int)`, `_Exit(int)`, and `abort(void)` intrinsics with interpreter-owned whole-program unwinding. `exit` and `_Exit` surface their one-time-evaluated status as Cust's program result; `abort` surfaces a recoverable `program aborted` error, and no intrinsic invokes host termination.

### Diagnostics and verification

- Added exact missing/incompatible declaration, arity, pointer/aggregate/void status-shape, user-definition precedence, nested control-flow, CLI, compiler-oracle, and non-evaluating constraint-aware `sizeof` coverage. Cust intentionally does not model `atexit`, stdio flushing, or signal delivery in this bounded slice.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.12.0`.
- Verified 1,271 tests at release preparation time: 1,137 interpreter tests, 98 deterministic fuzz-safety tests, and 36 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string/character/random/termination families, locale-sensitive behavior outside Cust's fixed ASCII/C-locale model, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.11.0 — 2026-08-02

### Language subset

- Added explicitly prototyped C11 `rand(void)` and `srand(unsigned int)` intrinsics with deterministic interpreter-owned state, default-seed equivalence, repeatable reseeding, a fixed `0..=32767` result range, user-definition precedence, exact call diagnostics, and non-evaluating `sizeof` validation.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.11.0`.
- Verified 1,259 tests at release preparation time: 1,126 interpreter tests, 98 deterministic fuzz-safety tests, and 35 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string/character/random families, locale-sensitive behavior outside Cust's fixed ASCII/C-locale model, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.10.0 — 2026-08-02

### Language subset

- Added exact-prototype C11 `tolower` and `toupper` intrinsics with user-definition precedence, one-time scalar evaluation, deterministic ASCII/C-locale conversion, and unchanged EOF or nonmatching unsigned-character values without host libc.

### Diagnostics and verification

- Added exact missing/incompatible declaration, arity, pointer/aggregate/void shape, and out-of-domain value diagnostics while direct and nested `sizeof` remain non-evaluating but constraint-aware. Review-driven regressions removed repeated nested-intrinsic validation and prove validation cost remains linear with expression depth.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.10.0`.
- Verified 1,255 tests at release preparation time: 1,122 interpreter tests, 98 deterministic fuzz-safety tests, and 35 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string/character families, locale-sensitive behavior outside Cust's fixed ASCII/C-locale model, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.9.0 — 2026-08-01

### Language subset

- Added exact-prototype C11 `isalnum`, `isalpha`, `isblank`, `iscntrl`, `isdigit`, `isgraph`, `islower`, `isprint`, `ispunct`, `isspace`, `isupper`, and `isxdigit` intrinsics with user-definition precedence, one-time scalar evaluation, deterministic ASCII/C-locale classes, canonical nonzero/zero results, and defined EOF plus unsigned-character-domain behavior without host libc.
- Added explicitly prototyped bounded C11 `strtok` over tracked mutable interpreter-owned character storage. It skips delimiter runs, writes token-ending NUL bytes in place, returns only nonempty first/continuation tokens, permits changed delimiter sets, and preserves continuation owner/lifetime identity without a host libc path.

### Diagnostics and verification

- Added exact character-classification missing/incompatible declaration, arity, pointer/aggregate/void shape, and out-of-domain value diagnostics; nested and direct `sizeof` calls remain non-evaluating but constraint-aware. Three focused interpreter tests, one registered warning-free native fixture, final independent approval, all 1,252 local tests, and both Docker gates pass.
- Added exact declaration, arity, type, qualification, null, read-only, expired-owner, ownerless-hidden-storage, two-dimensional-row, and independent 4,096-byte source/delimiter boundaries; runtime and nested `sizeof` paths share constraint checks while `sizeof` remains non-evaluating.
- Added eight focused interpreter regressions and one registered warning-free compiler-oracle fixture; all 1,249 local tests pass and independent final review found no blocking security or logic errors.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.9.0`.
- Verified 1,252 tests at release preparation time: 1,119 interpreter tests, 98 deterministic fuzz-safety tests, and 35 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string/classification families, locale-sensitive behavior outside Cust's fixed ASCII/C-locale model, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.8.0 — 2026-08-01

### Language subset

- Added explicitly prototyped bounded C11 `strncmp`, `strchr`, `strrchr`, `strpbrk`, `strspn`, `strcspn`, and `strstr` calls over interpreter-owned character storage. Calls evaluate arguments once in source order, normalize byte comparisons, enforce deterministic 4,096-byte bounds, and retain non-evaluating constraint-aware `sizeof`.
- Added explicitly prototyped bounded C11 `strcpy`, `strcat`, `strncat`, and `strncpy`. Mutable calls preflight destination capacity and overlap before mutation, preserve returned destination identity, enforce owner/lifetime/read-only and two-dimensional row boundaries, and implement exact NUL append, prefix, truncation, and padding behavior without a host libc path.

### Diagnostics and verification

- Added exact declaration, arity, scalar/pointer-shape, null, wrong-pointee, escaped-owner, unterminated-input, count/traversal-limit, capacity, overlap, read-only-write, row-boundary, and nested type-query coverage plus registered warning-free native compiler-oracle fixtures.
- Added review-driven regressions for lockstep early termination, nested non-evaluating call validation, scalar-character pointer identity, normalized byte handling, source-order one-time evaluation, complete append-range overlap, and early-NUL destination-capacity accounting.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.8.0`.
- Verified 1,241 tests at release preparation time: 1,108 interpreter tests, 98 deterministic fuzz-safety tests, and 35 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string families, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.7.0 — 2026-07-31

### Language subset

- Added the first bounded standard-library runtime slice: explicitly prototyped C11 `abs`, `labs`, and `llabs` calls execute inside Cust over its deterministic integer model, evaluate arguments once, and retain scalar `sizeof` metadata without evaluating calls.
- Added explicitly prototyped C11 `atoi`, `atol`, and `atoll` calls over interpreter-owned NUL-terminated character storage, with C whitespace/sign/decimal parsing, one-time pointer evaluation, a 4,096-byte scan bound, non-evaluating `sizeof`, and deterministic overflow handling.
- Added explicitly prototyped C11 `strcmp` and `strlen` calls over interpreter-owned character storage, with ordered one-time argument evaluation, unsigned-byte semantics, deterministic 4,096-byte traversal bounds, lexical sign results or exact lengths, and non-evaluating constraint-aware `sizeof`.

### Diagnostics and verification

- Added exact incompatible-prototype, missing-prototype, arity, scalar-shape, and minimum-integer absolute-value overflow boundaries plus a warning-free native compiler-oracle fixture.
- Added exact null, wrong-pointee, escaped-owner, unterminated-input, scan-limit, arity, incompatible-prototype, and integer-string-overflow boundaries plus warning-free native compiler-oracle parity.
- Added exact argument-specific pointer, owner, lifetime, termination, and traversal-limit diagnostics for string comparison and length calls, with review-driven unsigned-byte NUL handling and compiler-oracle registration coverage.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.7.0`.
- Verified 1,170 tests at release preparation time: 1,037 interpreter tests, 98 deterministic fuzz-safety tests, and 35 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls beyond the explicitly prototyped bounded integer/string families, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.6.0 — 2026-07-30

### Language subset

- Added C11 null preprocessing directives: active `#` and `%:` logical lines containing only preprocessing whitespace/comments have no effect across direct, spliced, conditional, and included-header routes.
- Added bounded `#pragma once` / `%:pragma once` for Linux file-aware sources, keyed by opened `(device, inode)` identity so repeated, symlinked, hard-linked, and recursive header paths share suppression without bypassing secure path resolution.
- Added bounded C11 `_Pragma("once")` for file-aware sources, including direct and object/function-macro-produced operators, split expanded-token sequences, standard encoding-prefix/quote/backslash destringization, inactive-group suppression, and reuse of opened-file `#pragma once` identity.

### Diagnostics and verification

- Added exact malformed/non-string/unsupported-pragma diagnostics, cumulative 1 MiB payload accounting, pre-allocation 8,192-token limits, and linear pending-operator processing regressions. Warning-free native parity, Windows GNU cross-target checking, 1,144 local tests, and both Docker gates pass.
- Added exact null-directive placement and pragma-name/trailing-token diagnostics; preserved secure include ordering and opened-file identity checks; and verified warning-free native fixtures plus independent review for each slice.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.6.0`.
- Verified 1,144 tests at release preparation time: 1,011 interpreter tests, 98 deterministic fuzz-safety tests, and 35 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, other pragma semantics, and host ABI layout/promotion rules remain unsupported.

## v0.5.0 — 2026-07-30

### Language subset

- Added direct-source C11 digraph punctuators: `<:`, `:>`, `<%`, `%>`, `%:`, and `%:%:` now behave as bracket, brace, directive/stringification, and token-pasting spellings while retaining exact preprocessing spelling metadata.
- Added bounded macro-expanded quoted-header operands for Linux file entry points: active `#include` / `%:include` directives may expand object-like, function-like, nested, and stringifying macros to exactly one ordinary string-literal header name before reusing the secure project-relative include path.
- Added bounded predefined `__FILE__` / `__LINE__` expansion with dynamic use-site lines, normalized primary/nested-header logical names, valid escaped token spelling, forwarding/stringification/conditional parity, and exact reserved-name diagnostics.
- Added bounded active C11 `#error` / `%:error` directives with unexpanded preprocessing-token messages, comment/whitespace normalization, physical splicing, conditional suppression, and included-header origin propagation.
- Added bounded C11 `#line` / `%:line` presumed source locations with direct and macro-expanded operands, optional ordinary source names, `__FILE__` / `__LINE__` integration, physical-location diagnostics, and include-local state restoration.

### Diagnostics and verification

- Preserved exact directive-placement and spelling-sensitive macro-redefinition diagnostics for digraph routes, added focused interpreter regressions plus a warning-free compiler-oracle fixture, and verified 1,113 local/Docker tests.
- Added exact non-string, multi-token, empty, wide-string, system-header, malformed-invocation, unsafe-path, nested-origin, and shared-expansion-budget diagnostics for macro-expanded includes; warning-free native project parity and 1,115 local/Docker tests pass.
- Added direct/forwarded/stringified/conditional and primary/nested-header predefined-macro coverage; warning-free native parity and 1,121 local/Docker tests pass.
- Added exact empty/message/source-context diagnostics for error directives plus a 1 MiB UTF-8 message bound whose overflow diagnostic uses bounded source context; 1,125 local/Docker tests pass.
- Added exact malformed/range/trailing-token diagnostics for line directives, an 8,192-token operand bound, a 4 KiB remapped-source-name bound with bounded overflow context, physical-splicing coverage, warning-free compiler-oracle parity, and 1,132 passing local/Docker tests.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.5.0`.
- Verified 1,132 tests at release time: 1,008 interpreter tests, 98 deterministic fuzz-safety tests, and 26 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- System headers, standard-library calls, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, and host ABI layout/promotion rules remain unsupported.

## v0.4.0 — 2026-07-29

### Language subset

- Added one-line object-like `#define` macros with deterministic nested expansion across declarations, enum/array integer constant expressions, and runtime expressions. Macro names inside comments and string/character literals remain untouched.
- Added bounded `#undef` directives: known definitions expire before subsequent tokens, unknown names are harmless, and removed names may be redefined with different replacement lists.
- Added bounded nested `#ifdef` / `#ifndef` conditional preprocessing with one `#else`, inactive-token/directive skipping, current macro-table definedness, and a 128-group nesting limit.
- Added bounded expression-form `#if` / `#elif` preprocessing with object-macro expansion, both `defined` spellings, ordered branch selection, C-style `intmax_t`/`uintmax_t` condition arithmetic, short-circuiting, and separate expression token/depth limits.
- Added global C11 physical-line splicing for LF and CRLF continuations before tokenization while retaining exact physical source positions.
- Added bounded function-like and variadic macros with balanced arguments, raw/prescanned substitution, replacement rescanning, temporary self-disable behavior, and reserved-identifier and compatible-redefinition checks.
- Added bounded project-relative quoted header inclusion on Linux with logical including-path search, shared macro state, guarded recursion, exact origins, secure beneath-root opening, and explicit depth/source-size limits.
- Added function-like macro stringification with raw arguments, whitespace normalization, escaping, `#` / `%:` spellings, variadic support, opaque preprocessing-token spelling, and expansion-separator preservation.
- Added function-like and object-like macro token pasting with raw adjacent arguments, placemarkers, chained reduction, exact one-token validation, rescanning, variadic support, and `##` / `%:%:` spelling metadata.

### Diagnostics and safety

- Added exact source-context diagnostics for recursive expansion, conflicting definitions, malformed parameters and invocations, reserved `__VA_ARGS__` misuse, invalid stringification/pasting operators and results, and bounded expansion depth/token/work/generated-byte exhaustion.
- Added exact diagnostics for malformed `#undef` and conditional directives, malformed `defined` expressions, invalid preprocessing integers and arithmetic, and inactive-branch structural errors.
- Added exact missing/system/macro-expanded/unsafe/non-regular header diagnostics, include-cycle/depth/source-size failures, and fail-closed quoted-inclusion behavior on unsupported platforms.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.4.0`.
- Added focused interpreter/CLI regressions and warning-free native compiler-oracle fixtures for every bounded preprocessing slice.
- Verified 1,110 tests at release time: 990 interpreter tests, 98 deterministic fuzz-safety tests, and 22 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Preprocessing still excludes system headers, macro-expanded include operands, ordinary-source bracket/brace digraph spellings, and `%:` directive introducers; `%:` / `%:%:` macro operators are supported, while secure quoted inclusion is Linux-only and fails closed elsewhere.
- Other unsupported areas include standard-library calls, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic function calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, and host ABI layout/promotion rules.

## v0.3.0 — 2026-07-27

### Language subset

- Added fixed local, global, file-static, and block-static `int[R][C]` / `char[R][C]` objects and named/anonymous aggregate fields with nested initialization, zero filling, deep-copy isolation, double-index scalar lvalues, const protection, exact per-dimension bounds diagnostics, and deterministic type queries.
- Added two-dimensional scalar-array typedef aliases and comma-separated direct/typedef-backed object declarations across local, global, and static storage.
- Added C array-parameter adjustment for direct and typedef-backed two-dimensional scalar arrays while preserving caller-owned storage, element type, fixed column width, qualification, forwarding, and bounds metadata.
- Added interpreter-owned pointer-to-row values for direct arrays and supported two-dimensional aggregate fields, including explicit `T (*row)[C]` objects/parameters, pointer-to-row typedef aliases, `T (*function(params))[C]` returns, row-scaled arithmetic/difference/equality/ordering, and double indexing through call/conditional/comma expressions.

### Diagnostics and safety

- Added exact type, width, rank, row/column bounds, const-discard/write, and escaped-local diagnostics for two-dimensional arrays and row pointers without exposing host addresses.
- Preserved explicit boundaries for variable-length arrays, arrays with more than two dimensions, aggregate-valued multidimensional elements, scalar-pointer flattening, and unsupported pointer-to-row declarator shapes.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.3.0`.
- Added focused interpreter regressions, deterministic row-expression/field property coverage, exact invalid fixtures, and warning-free native compiler-oracle fixtures.
- Verified 1,039 tests at release time: 929 interpreter tests, 98 deterministic fuzz-safety tests, and 12 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Unsupported areas include preprocessing/includes/macros, standard-library calls, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, and host ABI layout/promotion rules.

## v0.2.0 — 2026-07-25

### Language subset

- Added a deterministic aggregate model for named and anonymous structs/unions, enums, typedef-backed aggregate types, nested aggregate fields, scalar and aggregate arrays, pointer fields, aggregate parameters/returns, deep-copy assignment, designated initializers, and scalar/array/aggregate compound literals.
- Expanded declarations with comma-separated declarators and typedef aliases, inferred arrays, function prototypes, static/extern/thread-local storage-class syntax, `inline`/`_Noreturn`, `const`/`volatile`/`restrict`/`_Atomic`, `_Bool`, standard signed/unsigned/short/long scalar spellings, and C99 `__func__`.
- Expanded expressions with assignment and compound-assignment results, prefix/postfix increment and decrement, bitwise and shift operators, the conditional and comma operators, scalar/pointer/void casts, reverse subscripting, `sizeof`, `_Alignof`, and `_Static_assert`.
- Expanded the safe one-level typed pointer model with pointer-returning functions, scalar and aggregate array decay, aggregate/field/compound-literal storage roots, bounded arithmetic, same-array subtraction and ordering, address-of for supported lvalues, const-preserving conversions, and deterministic lifetime/type/bounds diagnostics.
- Added `do`/`while`, `switch`/`case`/`default` fallthrough, local function prototypes, block-scoped aggregate and enum definitions, C integer literal bases/suffixes, standard/numeric escapes, adjacent string-literal concatenation, and C line/block comments.

### Diagnostics and safety

- Added source-located diagnostics for unsupported preprocessor directives, floating/complex types, `void *`, function pointers/types, variadics, old-style parameters, `goto`/labels, `_Generic`, VLAs, multidimensional arrays, flexible array fields, bit-fields, forward declarations, and unsupported abstract declarator suffixes.
- Hardened parser boundaries with exact contextual diagnostics for malformed declarations, parameters, calls, control-flow headers, operators, type queries, array lengths/indexes/designators, initializers, casts, and unmatched delimiters.
- Added deterministic model-based coverage for pointer provenance/qualification/lifetime, aggregate and scalar expression classification, `_Bool` conversion boundaries, lexer/parser mutation matrices, comments/trivia, literals, adjacent strings, and first-error precedence.

### CLI, packaging, and verification

- Added `--tokens`, `--ast`, and `--max-steps`; retained `--version` with an exact `cust 0.2.0` release assertion.
- Added GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`) licensing so distributed and network-served modified versions remain open-source.
- Versioned the Cargo package and Docker Compose runtime/test images as `0.2.0`.
- Verified 984 tests at release time: 875 interpreter tests, 97 deterministic fuzz-safety tests, and 12 CLI, Docker, compiler-oracle, and repository tests.
- Continued using native C compilers only as external compatibility oracles for warning-free supported fixtures; Cust never delegates runtime execution to a native compiler.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Unsupported areas include preprocessing/includes/macros, standard-library calls, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic calls, VLAs and multidimensional arrays, flexible array members and bit-fields, `goto`, and host ABI layout/promotion rules.

## v0.1 — 2026-05-05

### Language subset

- Added function definitions and calls with local parameters, recursive and mutually recursive calls, arity diagnostics, undefined-function diagnostics, and a bounded call-depth guard.
- Added block-scoped variables, nested blocks, `if`/`else`, `while`, `for`, `break`, `continue`, empty statements, and expression statements.
- Added integer arithmetic and comparison coverage, unary plus/minus, logical `!`, `&&`, and `||` with C-style truth values and short-circuiting.
- Added `char` declarations/parameters/literals, one-dimensional `int`/`char` arrays, read-only NUL-terminated string literals, array parameters, indexed reads/writes, and deterministic negative/out-of-bounds diagnostics.
- Added the initial safe pointer model: scalar pointer declarations/reassignment, `&x`, `*p`, dereference assignment, null/out-of-scope diagnostics, pointer parameters, array/string decay to pointer arguments, `p[i]`, `&array[index]`, `&p[index]`, array-element pointers, pointer truthiness, and pointer equality/inequality against null and supported pointer targets.
- Added explicit diagnostics for unsupported pointer arithmetic, pointer ordering comparisons, and pointer-vs-nonzero-integer comparisons.

### Diagnostics and safety

- Parser diagnostics include source line/column metadata and context-specific messages for malformed function/parameter/call lists, missing semicolons, missing brackets/parens/braces, missing assignment operators, missing names/types, unmatched delimiters, unterminated blocks, malformed array lengths, and invalid `break`/`continue` placement.
- Lexer diagnostics include source-line/caret snippets for unexpected characters and out-of-range integer literals.
- Deterministic fuzz/property-style tests assert malformed generated programs and arbitrary bytes decoded through lossy UTF-8 do not panic the lexer/parser/interpreter path.
- Runtime diagnostics cover division by zero, loop execution limits, function call-depth limits, undefined variables/functions, array bounds, pointer null/out-of-scope/read-only/bounds failures, and unsupported pointer operations.

### CLI and tooling

- `cust <file.c>` interprets a source file and prints the integer returned by `main()`.
- `cust --version` prints the Cargo package version.
- `cust --tokens <file.c>` prints located lexer tokens without parsing or interpreting.
- `cust --ast <file.c>` prints a deterministic parser debug view without interpreting.
- `cust --max-steps N <file.c>` runs with an explicit total loop-iteration budget for bounded CLI execution.
- Docker Compose services use `pull_policy: build` so required cron verification rebuilds from the current checkout instead of silently reusing stale images.

### Test coverage and verification

- Added valid and invalid fixture corpora under `tests/fixtures/` for interpreter behavior, parser/lexer diagnostics, arrays/strings/pointers, and runtime errors.
- Added native C compiler compatibility tests for supported fixtures as an external oracle only; native compilers are never used as Cust's runtime path or implementation shortcut.
- Verified release gate on 2026-05-05:
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings`
  - `cargo test`
  - `docker compose run --rm test`
  - `docker compose run --rm cust`

### Known limitations

- Cust is not a full C implementation.
- Unsupported areas include the preprocessor, `#include`, standard-library calls such as `printf`, floating-point values, structs/unions/enums, prototypes/declarations separate from definitions, `void` functions, multiple pointer levels, pointer arithmetic, and general assignment expressions.
- The repository is licensed under the GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`).
