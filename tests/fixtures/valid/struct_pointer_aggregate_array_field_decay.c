struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[3];
    int bias;
};

int adjust_points(struct Point points[]) {
    points[0].x += 4;
    points[1].y = points[0].x + points[2].y;
    return points[0].x + points[1].y;
}

int bump_point(struct Point *point) {
    point->x += 6;
    return point->x + point->y;
}

int use_line(struct Line *line) {
    int total = adjust_points(line->points);      /* points[0].x = 5, points[1].y = 11, returns 16 */
    struct Point *last = &line->points[2];
    total += bump_point(last);                    /* points[2].x = 11, returns 17 */
    last = line->points + 1;
    total += last->x + last->y;                   /* 3 + 11 */
    line->points[0].y = total - line->bias;       /* 47 - 9 = 38 */
    return total + line->points[0].y;
}

int main(void) {
    struct Line line = {{{1, 2}, {3, 4}, {5, 6}}, 9};
    return use_line(&line) % 256;
}
