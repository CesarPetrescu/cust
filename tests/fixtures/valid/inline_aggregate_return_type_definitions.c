struct Pair {
    int x;
    int y;
} make_pair(int base);

struct Pair make_pair(int base) {
    struct Pair point = {base, base + 2};
    return point;
}

union Number {
    int value;
    char tag;
} pick_number(int flag) {
    union Number number = {flag ? 17 : 9};
    return number;
}

const struct Token {
    int value;
} make_token(void) {
    struct Token token = {23};
    return token;
}

int main(void) {
    struct Pair pair = make_pair(5);
    union Number number = pick_number(1);
    const struct Token token = make_token();
    return pair.x + pair.y + number.value + token.value + sizeof(struct Pair) + sizeof(union Number) + sizeof(struct Token);
}
