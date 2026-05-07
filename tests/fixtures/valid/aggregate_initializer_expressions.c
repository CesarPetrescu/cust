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

int sum_point(struct Point p) {
    p.x = p.x + 1;
    return p.x + p.y;
}

int main(void) {
    struct Point a = make_point(3, 4);
    struct Point b = make_point(a.y, a.x);
    const struct Point frozen = make_point(8, 9);
    union Number n = make_number(5);
    union Number copy = make_number(n.value + frozen.x);

    a.x = 10;
    n.tag = 7;

    return sum_point(b) + a.x + frozen.y + copy.value + n.value;
}
