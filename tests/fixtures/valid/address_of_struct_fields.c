struct Point {
    int x;
    int y;
};

struct Packet {
    struct Point anchor;
    int bias;
};

int bump_point(struct Point *p) {
    p->x += 10;
    (*p).y += 20;
    return p->x + p->y;
}

int add_one(int *slot) {
    *slot += 1;
    return *slot;
}

int main(void) {
    struct Point p = {1, 2};
    struct Point *pp = &p;
    int *px = &p.x;
    *px += 3;
    pp->y += 4;

    struct Point points[2] = {{5, 6}, {7, 8}};
    struct Point *second = &points[1];
    second->x += 30;
    (*second).y += 40;
    int *second_y = &points[1].y;
    *second_y += 2;

    struct Packet packet = {{9, 10}, 5};
    int *anchor_y = &packet.anchor.y;
    *anchor_y += 12;

    return p.x + p.y
        + points[1].x + points[1].y
        + packet.anchor.x + packet.anchor.y
        + bump_point(&points[0])
        + add_one(&packet.bias);
}
