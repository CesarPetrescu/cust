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
    int tail;
};

int bump(int *slot, int delta) {
    *slot = *slot + delta;
    return *slot;
}

int mutate_point(struct Point *point) {
    int *x = &point->x;
    int *y = &point->y;
    return bump(x, 5) + bump(y, 7);
}

int main(void) {
    struct Line line = {{{1, 2}, {3, 4}, {5, 6}}, 9};
    struct Point *mid = line.points + 1;
    int *mid_x = &mid->x;
    int total = bump(mid_x, 10);          /* points[1].x = 13, total = 13 */

    struct Box box = {{{{2, 3}, {4, 5}, {6, 7}}, 8}, 11};
    struct Point *nested = box.line.points + 2;
    int *nested_y = &nested->y;
    total = total + bump(nested_y, 20);   /* box.line.points[2].y = 27, total = 40 */

    total = total + mutate_point(line.points + 0); /* points[0] = {6,9}, total = 55 */

    return total + line.points[0].x + line.points[0].y + line.points[1].x + box.line.points[2].y;
}
