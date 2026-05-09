struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[2];
};

struct Box {
    struct Line inner;
};

int mutate(struct Point points[]) {
    points[0].x = 5;
    return points[0].x;
}

int main(void) {
    const struct Box box = {{{{1, 2}, {3, 4}}}};
    const struct Box *slot = &box;
    return mutate(slot->inner.points);
}
