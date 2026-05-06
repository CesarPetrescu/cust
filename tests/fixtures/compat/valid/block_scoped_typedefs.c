typedef int Word;

struct Pair {
    int left;
    char right;
};

typedef struct Pair Pair;

int main() {
    Word total = 2;

    {
        typedef char Word;
        Word letter = 'A';
        if (sizeof(Word) != sizeof(char)) {
            return 1;
        }
        total += letter - 'A';
    }

    {
        typedef Pair Word;
        Word pair;
        pair.left = 5;
        pair.right = 3;
        total += pair.left + pair.right;
    }

    Word still_int = 4;
    return total + still_int;
}
