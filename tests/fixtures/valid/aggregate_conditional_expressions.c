struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

struct Point make_point(int x, int y) {
    struct Point p = {x, y};
    return p;
}

union Number make_number(int value) {
    union Number n = {value};
    return n;
}

int main(void) {
    struct Point low = make_point(1, 2);
    struct Point high = make_point(10, 20);
    struct Point chosen = 1 ? high : low;
    struct Point other;
    other = 0 ? high : low;

    union Number left = make_number(3);
    union Number right = make_number(7);
    union Number selected = (left.value = 4, right);
    selected = 0 ? left : (right.value = 9, right);

    struct Point nested = (selected.value = selected.value + 1, chosen);

    return chosen.x + chosen.y + other.x + other.y + selected.tag + nested.y;
}
