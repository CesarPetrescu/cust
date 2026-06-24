int (add)(int left, int right);
static int (scale)(int value) {
    return value * 2;
}

struct Point {
    int x;
    int y;
};

struct Point (make_point)(int x, int y) {
    struct Point point = {x, y};
    return point;
}

int call_local(int value) {
    int (add)(int, int);
    extern struct Point (make_point)(int x, int y);
    struct Point made = make_point(value, scale(value));
    return add(made.x, made.y);
}

int (add)(int left, int right) {
    return left + right;
}

int (main)(void) {
    return call_local(3);
}
