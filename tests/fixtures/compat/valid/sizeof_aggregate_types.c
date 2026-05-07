struct Bytes {
    char left;
    char right;
};

union Number {
    int value;
    char tag;
};

int main(void) {
    int checks = 0;
    if (sizeof(struct Bytes) == sizeof(char) + sizeof(char))
        checks += 1;
    if (sizeof(union Number) == sizeof(int))
        checks += 2;
    if (sizeof(const union Number) == sizeof(union Number))
        checks += 4;
    if (sizeof(struct Bytes *) == sizeof(union Number *))
        checks += 8;
    return checks;
}
