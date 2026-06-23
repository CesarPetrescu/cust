struct Point {
    int x;
    int y;
};

struct Line {
    const struct Point points[2];
};

int main(void) {
    struct Line line = {{{1, 2}, {3, 4}}};
    struct Point replacement = {5, 6};
    line.points[1] = replacement;
    return line.points[1].x;
}
