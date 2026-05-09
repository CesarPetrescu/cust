struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[3];
    int bias;
};

int adjust_points(struct Point points[]) {
    points[1].x += 5;
    points[2].y = points[0].x + points[1].x;
    return points[1].x + points[2].y;
}

int bump_point(struct Point *point) {
    point->y += 7;
    return point->x + point->y;
}

int main(void) {
    struct Line line = {{{1, 2}, {3, 4}, {5, 6}}, 9};

    int total = adjust_points(line.points);      /* line.points[1].x = 8, line.points[2].y = 9, returns 17 */
    struct Point *middle = &line.points[1];
    total += bump_point(middle);                 /* y: 4 -> 11, returns 19 */
    middle = line.points + 2;
    total += middle->x + middle->y;              /* 5 + 9 */

    return (total + line.points[0].x + line.points[1].y + line.bias) % 256;
}
