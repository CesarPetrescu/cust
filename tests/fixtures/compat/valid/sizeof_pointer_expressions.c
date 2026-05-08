struct Pair {
    char x;
    char y;
};

union Byte {
    char value;
    char tag;
};

char *choose_char(char *text, int index) {
    return text + index;
}

struct Pair *choose_pair(struct Pair *pairs, int index) {
    return pairs + index;
}

union Byte *choose_byte(union Byte *bytes, int index) {
    return bytes + index;
}

int side_effect(void) {
    static int calls = 0;
    calls += 1;
    return calls;
}

char *side_effect_pointer(char *text) {
    side_effect();
    return text;
}

int main(void) {
    char text[3] = {'a', 'b', 0};
    struct Pair pairs[2] = {{1, 2}, {3, 4}};
    union Byte bytes[2] = {{5}, {.tag = 6}};

    char *text_ptr = text;
    struct Pair *pair_ptr = pairs;
    union Byte *byte_ptr = bytes;

    int total = 0;
    total += sizeof(*(text_ptr + 1));
    total += sizeof(*choose_char(text, 1));
    total += sizeof(*(pair_ptr + 1));
    total += sizeof(*choose_pair(pairs, 1));
    total += sizeof(*(byte_ptr + 1));
    total += sizeof(*choose_byte(bytes, 1));
    total += sizeof(*(&pairs[0].x));
    total += sizeof(*(&pairs[1].y));
    total += sizeof(*(1 ? text_ptr : choose_char(text, 0)));
    total += sizeof(*((side_effect(), text_ptr)));
    total += sizeof(*side_effect_pointer(text));

    return total + side_effect();
}
