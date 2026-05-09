struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[2];
};

int mutate(struct Point points[]) {
    points[0].x = 5;
    return points[0].x;
}

int main(void) {
    const struct Line line = {{{1, 2}, {3, 4}}};
    const struct Line *slot = &line;
    return mutate(slot->points);
}
