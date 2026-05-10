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
    int scalar_arrays = sizeof(int[3]) + sizeof(char[4]) + sizeof(const int[2]);
    int aggregate_arrays = sizeof(struct Pair[2]) + sizeof(Number[3]) + sizeof(Pair[1]);

    return scalar_arrays + aggregate_arrays;
}
