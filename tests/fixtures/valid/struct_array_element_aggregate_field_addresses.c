struct Point {
    int x;
    int y;
};

struct Segment {
    struct Point start;
    struct Point end;
};

int shift(struct Point *point, int dx, int dy) {
    point->x = point->x + dx;
    point->y = point->y + dy;
    return point->x + point->y;
}

int main(void) {
    struct Segment segments[2] = {{{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}};

    struct Point *second_start = &segments[1].start;
    struct Point *first_end = &segments[0].end;

    int total = shift(second_start, 2, 3); /* segments[1].start = {7, 9}; total = 16 */
    total = total + shift(first_end, 4, 5); /* segments[0].end = {7, 9}; total = 32 */

    return total
        + segments[1].start.x
        + segments[1].start.y
        + segments[0].end.x
        + segments[0].end.y;
}
