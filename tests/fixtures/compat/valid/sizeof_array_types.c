struct Pair {
    int left;
    char right;
};

union Number {
    int value;
    char tag;
};

typedef struct Pair Pair;
typedef union Number Number;

int main(void) {
    int checks = 0;
    if (sizeof(int[3]) == sizeof(int) * 3)
        checks += 1;
    if (sizeof(char[4]) == sizeof(char) * 4)
        checks += 2;
    if (sizeof(const int[2]) == sizeof(int[2]))
        checks += 4;
    if (sizeof(struct Pair[2]) == sizeof(struct Pair) * 2)
        checks += 8;
    if (sizeof(Number[3]) == sizeof(union Number) * 3)
        checks += 16;
    if (sizeof(Pair[1]) == sizeof(struct Pair))
        checks += 32;
    return checks;
}
