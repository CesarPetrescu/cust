struct Point {
    int x;
    int y;
};

int marker;

struct Point *pick(struct Point *points, int offset) {
    return points + offset;
}

int touch(void) {
    marker += 1;
    return marker;
}

int main(void) {
    struct Point points[4] = {{1, 2}, {3, 4}, {5, 6}, {7, 8}};
    struct Point other[4] = {{11, 12}, {13, 14}, {15, 16}, {17, 18}};
    int use_points = 1;

    struct Point *from_call = &pick(points, 1)[1];
    struct Point *from_conditional = &(use_points ? points : other)[3];
    struct Point *from_comma = &(touch(), points)[0];

    from_call->x += 20;
    from_conditional->y += 30;
    from_comma->x += 40;

    int call_index = from_call - points;
    int conditional_index = from_conditional - points;
    int comma_index = from_comma - points;

    return points[2].x + points[3].y + points[0].x + marker
        + call_index + conditional_index + comma_index;
}
