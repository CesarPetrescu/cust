struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[3];
};

int sum(struct Point p) {
    return p.x + p.y;
}

int main(void) {
    struct Line line = {{{1, 2}, {3, 4}, {5, 6}}};
    struct Point replacement = {7, 8};
    struct Point returned = (line.points[1] = replacement);

    replacement.x = 20;
    line.points[2] = (struct Point){9, 10};

    struct Line *slot = &line;
    slot->points[0] = (struct Point){11, 12};

    return sum(returned) + line.points[0].x + line.points[0].y
        + line.points[1].x + line.points[1].y
        + line.points[2].x + line.points[2].y;
}
