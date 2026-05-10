struct Point {
    int x;
};

struct Line {
    struct Point points[3];
};

struct Box {
    struct Line line;
};

int same_point(struct Point *left, struct Point *right) {
    return left == right;
}

int main(void) {
    struct Line line = {{{2}, {4}, {6}}};
    struct Box box = {{{{3}, {5}, {7}}}};
    struct Point *start = line.points;
    struct Point *first = &line.points[0];
    struct Point *middle = &line.points[1];
    int score = 0;
    if (start == first) {
        score += 1;
    }
    if (first != middle) {
        score += 2;
    }
    if ((line.points + 2) == &line.points[2]) {
        score += 4;
    }
    if (same_point(box.line.points, &box.line.points[0])) {
        score += 8;
    }
    if ((box.line.points + 1) != box.line.points) {
        score += 16;
    }
    return score;
}
