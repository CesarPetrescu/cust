int align_result(void) {
    return (_Alignof(enum ReturnAlign { RETURN_ALIGN = 4 }) == _Alignof(enum ReturnAlign))
        + RETURN_ALIGN;
}

int main(void) {
    int total = 0;
    total = total + (_Alignof(enum LocalAlign { LOCAL_ALIGN = 6 }) == _Alignof(enum LocalAlign));
    total = total + LOCAL_ALIGN;

    int check = _Alignof(enum DeclAlign { DECL_ALIGN = 9 }), mirror = DECL_ALIGN;
    total = total + (check == _Alignof(enum DeclAlign)) + mirror;

    return total + align_result();
}
