struct Point {
    int x;
};

struct Line {
    struct Point points[3];
};

struct Box {
    struct Line line;
};

int order_points(struct Point *first, struct Point *middle, struct Point *last) {
    struct Point *same = middle;
    int score = 0;
    if (first < middle) {
        score += 1;
    }
    if (middle <= same) {
        score += 2;
    }
    if (last > middle) {
        score += 4;
    }
    if (last >= first) {
        score += 8;
    }
    return score;
}

int main(void) {
    struct Line line = {{{2}, {4}, {6}}};
    struct Box box = {{{{3}, {5}, {7}}}};
    struct Point *start = line.points;
    struct Point *end = &line.points[2];
    int direct = (start < end) + ((line.points + 1) <= end) * 2 + (end > line.points) * 3;
    int nested = order_points(box.line.points, box.line.points + 1, &box.line.points[2]);
    return direct + nested;
}
