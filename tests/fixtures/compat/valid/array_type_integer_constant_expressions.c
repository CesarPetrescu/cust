enum Lengths {
    BASE_LEN = 2,
    EXTRA_LEN = 1
};

typedef int Count;

struct Pair {
    int x;
    char y;
};

typedef struct Pair Pair;

int main(void) {
    int total = 0;

    total += sizeof(int[BASE_LEN + EXTRA_LEN]) == (BASE_LEN + EXTRA_LEN) * sizeof(int);
    total += sizeof(char[sizeof(char) + 2]) == 3 * sizeof(char);
    total += sizeof(const Count[BASE_LEN ? 2 : 4]) == 2 * sizeof(Count);
    total += sizeof(Pair[BASE_LEN]) == BASE_LEN * sizeof(Pair);
    total += _Alignof(int[BASE_LEN + EXTRA_LEN]) == _Alignof(int);
    total += _Alignof(Pair[EXTRA_LEN + 1]) == _Alignof(Pair);

    return total;
}
