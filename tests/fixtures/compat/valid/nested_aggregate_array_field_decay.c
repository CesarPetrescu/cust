struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[3];
    int bias;
};

struct Box {
    struct Line inner;
    int scale;
};

int adjust_points(struct Point points[]) {
    points[0].x += 2;
    points[1].y = points[0].x + points[2].y;
    return points[0].x + points[1].y;
}

int bump_point(struct Point *point) {
    point->y += 5;
    return point->x + point->y;
}

int use_box(struct Box *box) {
    int total = adjust_points(box->inner.points);      /* p0.x = 3, p1.y = 9, returns 12 */
    struct Point *last = &box->inner.points[2];
    total += bump_point(last);                         /* p2.y = 11, returns 16 */
    last = box->inner.points + 1;
    total += last->x + last->y;                        /* 3 + 9 */
    box->inner.points[0].y = total - box->inner.bias;  /* 40 - 7 = 33 */
    return total + box->inner.points[0].y + box->scale;
}

int main(void) {
    struct Box box = {{{{1, 2}, {3, 4}, {5, 6}}, 7}, 8};
    return use_box(&box) % 256;
}
