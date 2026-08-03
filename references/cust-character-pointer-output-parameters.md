# Cust Safe Character Pointer Output Parameters

## Scope

This note records the 2026-08-03 bounded prerequisite for C11 `strtol`/`strtoul` `endptr`. Cust intentionally supports only direct unqualified `char **` function parameters; it does not generalize the interpreter to arbitrary pointer-to-pointer objects.

## Parser and signature model

- `ParamKind::CharacterPointerOutput` distinguishes `char **` from ordinary `char *` in prototypes and definitions.
- Reject qualifiers on the character type, either pointer level, and qualifier-carrying aliases before admitting the narrow form. This includes `const`, `volatile`, `restrict`, and `_Atomic` routes.
- Preserve the existing exact deeper-pointer diagnostic for `char ***` and other unsupported two-level pointer types.
- Store the parameter marker in `FunctionSignature` so prototypes, definitions, and forwarding calls compare the full bounded shape.

## Runtime model

- `CharacterPointerOutput::Slot` names an existing mutable `char *` variable slot by `(scope_id, name)`; it does not hold a host address.
- `CharacterPointerOutput::Null` models literal null for equality/truthiness and forwarding.
- Binding accepts `&name` only when `name` resolves lexically to a mutable `char *` slot whose pointee is not const. Local, global, file-static, and block-static slots are supported.
- Forwarding a `char **` parameter copies the slot identity. Writes through `*out` update the original slot and preserve the assigned pointer's owner, lifetime, pointee type, and const metadata.
- Validate the slot's lexical owner before reads, writes, forwarding, truthiness/equality, and non-evaluating classification.

## Safety boundaries and recurring pitfalls

The implementation uses an internal scalar placeholder only to integrate with existing scope storage. Every operation that could observe or mutate that placeholder must be intercepted:

- direct scalar evaluation;
- outer-slot assignment and compound assignment;
- prefix/postfix increment and decrement, including beneath `sizeof`;
- taking `&out`, including beneath `sizeof`;
- `sizeof(out)` must report pointer size, not the placeholder's `_Bool` size.

Lexical lookup must stop at enum constants. An inner `enum { out = 0 };` must not fall through to an outer output parameter or pointer variable in either argument binding or address-of evaluation.

Do not infer qualification from only a `const` boolean. The parser's generic qualifier consumer also accepts `volatile`, `restrict`, and `_Atomic`; capture qualifier presence and source location before consuming it, including alias-carried qualification.

## Verification

Focused regression family:

```bash
cargo test --test interpreter character_pointer_output -- --nocapture
cargo test --test interpreter address_of_respects_enum_shadowing_over_character_pointer_outputs -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

The native fixture is an oracle only. It covers null/non-null output writes, forwarding, `*out`, equality/truthiness, and non-evaluating assignment while avoiding host-address use in Cust.

After review fixes, rerun the complete canonical local and Docker gate. Any code/test edit after review invalidates earlier gate evidence.

## Follow-up

Bounded explicitly prototyped `strtol`/`strtoul` can now use this output-slot model. The intrinsic package must still cover bases 0 and 2..36, no-digit/partial/overflow behavior, exact absent/null/non-null `endptr` writes, one-time source-order evaluation, user-definition precedence, owner/lifetime safety, nested non-evaluating validation, and warning-free native relationships.
