struct Point {
    int x;
    int y;
};

struct Segment {
    struct Point start;
    struct Point end;
};

struct Drawing {
    struct Segment segments[2];
    int scale;
};

struct Box {
    struct Drawing drawing;
};

int shift(struct Point *point, int dx, int dy) {
    point->x = point->x + dx;
    point->y = point->y + dy;
    return point->x + point->y;
}

int main(void) {
    struct Drawing drawing = {{{{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}}, 9};
    struct Box box = {{{{{10, 11}, {12, 13}}, {{14, 15}, {16, 17}}}, 18}};

    struct Point *second_start = &drawing.segments[1].start;
    struct Point *nested_end = &box.drawing.segments[0].end;

    int total = shift(second_start, 2, 3); /* drawing.segments[1].start = {7, 9}; total = 16 */
    total = total + shift(nested_end, 4, 5); /* box.drawing.segments[0].end = {16, 18}; total = 50 */

    return total
        + drawing.segments[1].start.x
        + drawing.segments[1].start.y
        + box.drawing.segments[0].end.x
        + box.drawing.segments[0].end.y;
}
