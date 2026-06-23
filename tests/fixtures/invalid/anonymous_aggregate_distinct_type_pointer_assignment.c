int main(void) {
    struct { int value; } left = {1};
    struct { int value; } *right = &left;
    return right->value;
}
