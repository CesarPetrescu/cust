struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[2];
    int tag;
};

int sum_first(struct Line line) {
    return line.points[0].x + line.points[1].y + line.tag;
}

int main(void) {
    struct Line line = {{{1, 2}, {.x = 3, .y = 4}}, 5};

    line.points[1].x += 2;
    line.points[0].y = line.points[1].x;

    return line.points[0].x
        + line.points[0].y
        + line.points[1].x
        + line.points[1].y
        + sum_first(line);
}
