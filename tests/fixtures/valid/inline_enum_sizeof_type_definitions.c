int value_from_sizeof_return(void) {
    return sizeof(enum ReturnSize { RETURN_SIZE = 5 }) == sizeof(enum ReturnSize)
        ? RETURN_SIZE
        : 0;
}

int main(void) {
    int first_size = sizeof(enum LocalSize { LOCAL_SIZE = 7 });
    int local_matches = first_size == sizeof(enum LocalSize);
    int decl_size = sizeof(enum DeclSize { DECL_SIZE = 11 }), mirror = DECL_SIZE;
    int decl_matches = decl_size == sizeof(enum DeclSize);

    return (local_matches ? LOCAL_SIZE : 0) + mirror + value_from_sizeof_return() + decl_matches;
}
