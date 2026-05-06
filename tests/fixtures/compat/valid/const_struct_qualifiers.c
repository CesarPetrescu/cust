struct Point {
    int x;
    char y;
};

int sum_const_pointer(const struct Point *point) {
    return point->x + point->y;
}

int pass_const_pointer(const struct Point *point) {
    const struct Point *again = point;
    return again->x - again->y;
}

int main() {
    struct Point point;
    point.x = 9;
    point.y = 4;
    const struct Point *view = &point;
    if (view->x != 9) return 1;
    if (view->y != 4) return 2;
    if (sum_const_pointer(view) != 13) return 3;
    if (pass_const_pointer(&point) != 5) return 4;
    return 0;
}
