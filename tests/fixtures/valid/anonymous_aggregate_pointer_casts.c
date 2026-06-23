int main(void) {
    int null_struct = (struct { int x; } *)0 == 0;
    int null_const_union = (const union { char tag; } *)0 == 0;
    int sizeof_struct_pointee = sizeof(*(struct { char tag; } *)0) == sizeof(char);
    return null_struct + null_const_union + sizeof_struct_pointee;
}
