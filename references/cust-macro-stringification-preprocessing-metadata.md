# Function-like macro stringification preprocessing metadata

Cust's bounded C11 `#` implementation requires preprocessing-phase metadata that cannot be reconstructed from semantic tokens alone.

## Token model

Each `LocatedToken` carries:

- its semantic `Token` kind;
- exact preprocessing spelling after physical line splicing and comment replacement;
- whether preprocessing whitespace separated it from the previous token;
- its physical source location/origin.

Replacement-list lexing also accepts opaque preprocessing tokens such as `1e+2`, `L"wide"`, oversized escape spellings, `#`, and `%:`. Opaque tokens are decoded only if they survive expansion into ordinary Cust syntax; stringification consumes their original spelling directly.

## Stringification

For `# parameter` and `%: parameter`:

1. use the raw, unprescanned argument;
2. ignore leading/trailing whitespace;
3. collapse internal whitespace/comment runs to one space;
4. escape quotes and backslashes from string/character-literal spellings in the generated source spelling;
5. retain unescaped text as the runtime string value;
6. rescan the generated string token normally.

A parameter used only by `#` is not prescanned. A parameter used both normally and by `#` retains separate raw and prescanned representations.

## Separator propagation

Macro expansion returns both output tokens and a `trailing_separation` flag. This is necessary when an expansion ends empty but whitespace must affect a later token across argument, replacement, nested invocation, or source seams.

Before ordinary argument prescan, clear only the raw argument's first token `separated` flag so invocation-leading whitespace is trimmed. Preserve the untouched raw argument for `#`. During substitution, retain any first-token separation created by expansion and combine it with the replacement parameter position and pending separator.

When pending separation is consumed by an already-separated token, clear pending state unconditionally. Do not use short-circuit `replacement_separated || mem::take(&mut pending)`: if the left side is true, `take` is skipped and separation leaks to a later source token.

Focused regression shapes:

```c
#define EMPTY
#define STR(x) #x
#define XSTR(x) STR(x)
#define ID(x) x
#define WRAP(x) pre x post
#define A EMPTY a
#define CAT(x) b+x

XSTR(z+ID( a))       /* "z+a" */
XSTR(WRAP(EMPTY)+z)  /* "pre post+z" */
XSTR(z+ID( EMPTY)+a) /* "z++a" */
XSTR(CAT(A))         /* GCC: "b+ a" */
```

GCC and Clang differ on the final expansion-created leading-separator spelling, so that route remains an interpreter/GCC-focused metadata regression rather than compiler-neutral `c_compat` fixture content. The other routes are warning-free under Cust, GCC, and Clang.

## Bounds and diagnostics

- A translation-unit-wide 1 MiB generated-string byte budget is separate from macro token/work limits.
- `#` in object-like replacement lists reports a function-like-only diagnostic.
- `#` not followed by a formal parameter or `__VA_ARGS__` reports at the operator.
- `##` and `%:%:` remain explicitly unsupported and report at the first operator token.
- Public `LocatedToken` equality intentionally ignores preprocessing-only spelling/separation metadata so existing token/AST tests compare semantic output.
