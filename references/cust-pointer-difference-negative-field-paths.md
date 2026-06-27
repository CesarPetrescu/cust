# Pointer difference diagnostics through field-backed array paths

Date: 2026-06-27

Cust supports pointer subtraction only for pointers into the same interpreter-owned array-like storage. This coverage locks in the negative side for less-traveled field-backed array paths:

- embedded aggregate-array fields selected from the same containing object but different fields (`line.right - line.left`)
- scalar array fields selected from different aggregate objects (`right.values - left.values`)
- anonymous aggregate-array fields selected from different aggregate objects (`right.items - left.items`)

Expected diagnostic: `cannot subtract pointers to different arrays`.

Focused coverage passed immediately because existing pointer identity/storage metadata already distinguishes these array owners/field paths. Treat this as conformance/diagnostic coverage, not a runtime fix. Keep compiler-oracle coverage out of this invalid-fixture slice; native C rejects or leaves some cross-object pointer subtraction forms outside warning-free supported oracle use.
