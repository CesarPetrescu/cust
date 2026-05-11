typedef int Count;

struct Pair {
    char tag;
    int value;
};

typedef struct Pair Pair;

union Number {
    char tag;
    int value;
};

int main(void) {
    int total = 0;
    total = total + _Alignof(char);
    total = total + _Alignof(_Bool);
    total = total + _Alignof(int);
    total = total + _Alignof(const Count);
    total = total + _Alignof(int *);
    total = total + _Alignof(char[4]);
    total = total + _Alignof(struct Pair);
    total = total + _Alignof(union Number);
    total = total + _Alignof(Pair[2]);
    return total;
}
