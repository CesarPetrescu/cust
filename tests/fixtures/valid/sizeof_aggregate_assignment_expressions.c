struct Pair {
    int x;
    char tag;
};

union Number {
    int value;
    char tag;
};

int main(void) {
    struct Pair left = {1, 2};
    struct Pair right = {3, 4};
    union Number number = {5};
    union Number replacement = {7};
    int marker = 0;
    int ok = 0;

    ok += sizeof((left = right)) == sizeof(struct Pair);
    ok += left.x == 1;
    ok += sizeof((marker = marker + 1, left = right)) == sizeof(struct Pair);
    ok += marker == 0;
    ok += left.tag == 2;
    ok += sizeof((number = replacement)) == sizeof(union Number);
    ok += number.value == 5;

    return ok;
}
