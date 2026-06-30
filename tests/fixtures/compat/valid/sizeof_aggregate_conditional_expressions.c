struct Pair {
    char tag;
    int value;
};

union Tiny {
    char tag;
};

int main(void) {
    int marker = 0;
    int choose_right = 0;
    struct Pair left = {1, 2};
    struct Pair right = {3, 4};
    union Tiny a = {'a'};
    union Tiny b = {'b'};
    int total = 0;

    total += sizeof(choose_right ? left : right) == sizeof(struct Pair);
    total += sizeof((marker = marker + 1, choose_right ? left : right)) == sizeof(struct Pair);
    total += sizeof(1 ? (left = right) : right) == sizeof(struct Pair);
    total += sizeof(choose_right ? a : b) == sizeof(union Tiny);
    total += marker == 0;
    total += left.tag == 1 && left.value == 2;

    return total;
}
