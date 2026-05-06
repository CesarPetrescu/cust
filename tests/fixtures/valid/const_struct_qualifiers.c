struct Point {
    int x;
    char y;
};

int sum_const_pointer(const struct Point *point) {
    return point->x + point->y;
}

int shadow_copy(const struct Point point) {
    struct Point local;
    local = point;
    local.x += 3;
    return local.x + point.x;
}

int main() {
    struct Point mutable;
    mutable.x = 5;
    mutable.y = 7;

    const struct Point zero;
    const struct Point *view = &mutable;
    struct Point * const stable = &mutable;
    stable->x += 1;

    return zero.x + zero.y + view->x + view->y + sum_const_pointer(&mutable) + shadow_copy(mutable);
}
