struct Point {
    int x, y;
    char tag, code;
};

struct Segment {
    struct Point start, end;
    int weights[2], offsets[2];
};

struct Cursor {
    int *head, *tail;
    const int *view, *limit;
};

union Number {
    int value, other;
};

int main(void) {
    struct Segment segment = {{1, 2, 'A', 'B'}, {3, 4, 'C', 'D'}, {5, 6}, {7, 8}};
    int values[4] = {10, 20, 30, 40};
    struct Cursor cursor = {values, values + 3, values + 1, values + 2};
    union Number number;
    number.other = 9;

    return *cursor.head + *cursor.tail + *cursor.view + *cursor.limit
        + segment.start.x + segment.start.y + segment.end.x + segment.end.y
        + segment.weights[1] + segment.offsets[0]
        + segment.start.code - segment.start.tag
        + number.other;
}
