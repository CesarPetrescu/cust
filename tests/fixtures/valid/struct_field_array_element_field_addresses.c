struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[3];
    int bias;
};

struct Box {
    struct Line line;
};

int bump(int *slot, int delta) {
    *slot = *slot + delta;
    return *slot;
}

int main(void) {
    struct Line line = {{{1, 2}, {3, 4}, {5, 6}}, 7};
    struct Box box = {{{{8, 9}, {10, 11}, {12, 13}}, 14}};

    int *middle_x = &line.points[1].x;
    int *middle_y = &line.points[1].y;
    int *nested_y = &box.line.points[2].y;

    int total = bump(middle_x, 20);    /* line.points[1].x = 23, total = 23 */
    total = total + bump(middle_y, 30); /* line.points[1].y = 34, total = 57 */
    total = total + bump(nested_y, 40); /* box.line.points[2].y = 53, total = 110 */

    return total
        + line.points[0].x
        + line.points[1].x
        + line.points[1].y
        + box.line.points[2].y;
}
