typedef int Count;
typedef char *Text;

struct Pair {
    int left;
    char right;
};

typedef struct Pair Pair;
typedef struct Pair *PairPtr;

int main() {
    return (sizeof(const char) == sizeof(char))
        + (sizeof(const Count) == sizeof(Count))
        + (sizeof(const int *) == sizeof(int *))
        + (sizeof(const char *) == sizeof(char *))
        + (sizeof(const Text) == sizeof(Text))
        + (sizeof(const Pair) == sizeof(Pair))
        + (sizeof(const PairPtr) == sizeof(PairPtr));
}
