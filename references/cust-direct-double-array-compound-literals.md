# Direct double array compound literals

The 2026-08-25 package enables direct and typedef-backed one-dimensional `double` array compound literals by removing the two scalar-double rejection guards in `Parser::parse_cast()`. No new AST or runtime variant is required: existing `Expr::ArrayLiteral`, `ArrayInitializer`, hidden `Rc<RefCell<Vec<ScalarValue>>>` storage, `PointeeType::Scalar(CType::Double)`, and scope-owner metadata already preserve the complete supported behavior.

## TDD and coverage

- RED 1: `(double[3]){...}` failed with `double array compound literals are not supported`; removing only the direct scalar guard made fixed direct literals GREEN.
- RED 2: a `typedef double Row[4];` literal still failed at the alias-expanded array guard; removing only that guard made typedef-backed literals GREEN.
- Fixed/inferred lengths, positional/designated initialization, zero fill, mutation, pointer arithmetic, `_Generic`, `_Alignof`, full-object/element `sizeof`, and one-time evaluated initialization reuse the ordinary scalar-array path.
- Side-effecting initializers beneath `sizeof` remain non-evaluating. Native compiler-oracle fixtures should keep such side effects out of unevaluated operands because Clang may warn under `-Wunevaluated-expression`; prove non-evaluation in a Cust-only fixture instead.
- Direct and typedef-backed const discard/write and escaped hidden-root pointers retain exact diagnostics.

## Safety closure

Test both evaluated and non-evaluating contexts after removing a feature guard. In this slice, multidimensional literals still stop at `reject_multidimensional_array_cast_suffix()`, whole-array address/pointer-to-row construction still reports `invalid address-of target`, union-backed double-array decay still reports `double pointers are not supported`, and all exactly prototyped raw-memory intrinsics still report unsupported double object storage.

Register the warning-free native fixture explicitly in `tests/c_compat.rs` and run the actual `supported_programs_match_c_compiler_exit_codes` test; fixture-name filtering runs zero tests and is not verification.