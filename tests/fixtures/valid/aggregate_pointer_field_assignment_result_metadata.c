struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

struct PointCursor {
    const struct Point *points;
};

struct NumberCursor {
    const union Number *numbers;
};

int main(void) {
    struct Point points[4] = {{1}, {2}, {3}, {4}};
    union Number numbers[4] = {{5}, {6}, {7}, {8}};
    struct PointCursor point_cursor = {points};
    struct NumberCursor number_cursor = {numbers};
    struct NumberCursor *number_view = &number_cursor;
    int marker = 0;

    const struct Point *direct = (point_cursor.points = points) + 1;
    const struct Point *literal =
        (0 ? points : (((struct PointCursor){points}).points = points) + 2);
    const union Number *arrow =
        (marker += 1, (number_view->numbers = numbers) + 1);

    return direct->value + literal->value + arrow->value + marker;
}
