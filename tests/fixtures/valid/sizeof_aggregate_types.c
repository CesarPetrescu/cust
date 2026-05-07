struct Bytes {
    char left;
    char right;
};

union Number {
    int value;
    char tag;
};

typedef union Number Number;

int main(void) {
    int struct_size = sizeof(struct Bytes);
    int union_size = sizeof(union Number);
    int const_union_size = sizeof(const union Number);
    int typedef_union_size = sizeof(Number);
    int pointer_size = sizeof(struct Bytes *) + sizeof(union Number *);

    return struct_size + union_size + const_union_size + typedef_union_size + pointer_size;
}
