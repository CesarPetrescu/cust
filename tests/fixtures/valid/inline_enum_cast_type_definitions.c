int value_from_cast(void) {
    return (enum InlineReturn { RETURN_VALUE = 6 })0 + RETURN_VALUE;
}

int main(void) {
    (void)(enum InlineExpr { EXPR_VALUE = 4 })0;
    int local = EXPR_VALUE;
    int from_decl = (enum InlineDecl { DECL_VALUE = 9 })0, mirror = DECL_VALUE;
    return local + mirror + value_from_cast() + from_decl;
}
