# Addressable Compound Literals

Cust now supports a scoped first pass of C99 compound-literal addressability.

## Supported syntax

- Scalar compound literals may be addressed: `&(int){7}`, `&(char){'a'}`, and scalar typedef variants after alias resolution.
- Aggregate compound literals may be addressed: `&(struct Point){.x = 1}` and aggregate typedef variants after alias resolution.
- The resulting pointers flow through the existing one-level pointer model, including pointer parameters, dereference reads/writes, `->` field access for aggregate pointers, and pointee type/const conversion checks.

## Storage and lifetime model

When an addressable scalar or aggregate compound literal is evaluated, Cust creates hidden mutable storage in the current interpreter scope and returns an interpreter-owned pointer to that hidden object. This mirrors Cust's existing hidden current-scope storage for aggregate-array compound literals while avoiding host addresses. The hidden object expires with the surrounding Cust block/function scope, so existing out-of-scope pointer diagnostics remain applicable if such a pointer escapes.

Scalar/aggregate compound literals without address-of remain rvalue expressions. Array compound literals continue to be pointer-valued expressions backed by mutable storage.

## Out of scope

- Pointer-to-pointer compound literals remain unsupported because Cust's pointer-to-pointer model is intentionally absent.
- Native C ABI layout and padding remain irrelevant; aggregate pointers reference Cust field maps.
- Static-duration file-scope compound literal storage is not modeled separately; Cust's global scope already persists for top-level expression evaluation paths that can create hidden storage.
