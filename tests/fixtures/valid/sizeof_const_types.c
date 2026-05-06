typedef int Count;
typedef char *Text;

struct Pair {
    int left;
    char right;
};

typedef struct Pair Pair;
typedef struct Pair *PairPtr;

int main() {
    return sizeof(const int)
        + sizeof(const char)
        + sizeof(const Count)
        + sizeof(const int *)
        + sizeof(const char *)
        + sizeof(const Text)
        + sizeof(const Pair)
        + sizeof(const PairPtr);
}
