typedef int Count, *CountPtr, Counts[3];

typedef struct Point {
    int x;
    int y;
} Point;

Point make_point(int base) {
    return (Point){base, base + 1};
}

int main(void) {
    Count total = 0;
    Counts values = {1, 2, 3};
    CountPtr cursor = values;
    total += cursor[2];

    for (enum { START = 2 } i = START; i < 4; i++) {
        total += i;
    }

    struct { int x; int y; } *slot = 0, point = {4, 5};
    slot = &point;
    total += slot->x + point.y;

    const struct { int value; int extra; } rows[2] = {{3, 4}, {5, 6}}, *view = rows + 1;
    total += view->value + rows[0].extra;

    enum { BOOST = 4 } boost = BOOST;
    Point made = make_point(7);
    total += boost + made.x + made.y;

    return total;
}
