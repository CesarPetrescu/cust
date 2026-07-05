struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

struct Segment {
    struct Point points[3];
    int weights[2];
};

struct Board {
    union Number numbers[2];
};

struct Segment global_segment = {
    .points[2].x = 5,
    .points[0].y = 7,
    .points[1] = {2, 3},
    .weights[1] = 4,
};

int sum_segment(struct Segment segment) {
    return segment.points[0].x + segment.points[0].y +
           segment.points[1].x + segment.points[1].y +
           segment.points[2].x + segment.points[2].y +
           segment.weights[0] + segment.weights[1];
}

int main(void) {
    struct Segment local = {
        .points[0].x = 11,
        .points[0].y = 13,
        .points[2] = {.x = 17, .y = 19},
        .weights[0] = 1,
    };
    struct Board board = {
        .numbers[1].tag = 'C',
    };

    return sum_segment(global_segment) + sum_segment(local) + board.numbers[1].tag - 'A';
}
