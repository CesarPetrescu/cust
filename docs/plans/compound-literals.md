# Compound Literal Model

Cust supports a deliberately scoped C99-style compound literal subset:

- Supported aggregate syntax: `(struct Name){...}`, `(union Name){...}`, and aggregate typedef aliases such as `(Point){.x = 1}`.
- Supported scalar syntax: `(int){expr}`, `(char){expr}`, and scalar typedef aliases such as `(Count){expr}`. Scalar compound literals accept exactly one scalar initializer expression plus an optional trailing comma and behave like rvalue scalar expressions in Cust's existing integer storage model.
- Aggregate brace bodies reuse the existing aggregate initializer parser, including positional entries, field designators, nested aggregate initializers, scalar array-field initializers, and union first/active-field behavior.
- Scalar brace bodies reuse the existing braced scalar initializer parser, preserving side effects exactly once in evaluated contexts and remaining non-evaluating under `sizeof`.
- Compound literals are rvalue expressions in Cust's interpreter model. Aggregates can initialize aggregate variables, feed same-type aggregate assignment expressions, be passed to by-value aggregate parameters, be returned from aggregate-returning functions, appear in aggregate `?:` / comma expressions, and expose scalar/nested struct fields through `.` field access. Scalars can appear in arithmetic/comparison/conditional/call-argument contexts and `sizeof((char){...})` reports Cust's deterministic char size without evaluating the initializer.
- Cust does not model C compound literal object address/lifetime yet. Taking the address of a compound literal, mutating it as an lvalue, or using array compound literals is intentionally out of scope until a storage/lifetime design is written.
- Native C compilers remain test oracles only. Compatibility fixtures compare exit codes for supported expression behavior without relying on native ABI padding or object addresses.
