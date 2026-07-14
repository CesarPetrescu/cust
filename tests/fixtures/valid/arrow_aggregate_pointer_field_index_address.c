struct Point {
    int x;
};

union Number {
    int value;
    char tag;
};

struct Cursor {
    struct Point * const points;
    union Number * const numbers;
};

int main(void) {
    struct Point points[3] = {{1}, {2}, {3}};
    union Number numbers[3] = {{4}, {5}, {6}};
    struct Cursor cursor = {points, numbers};
    const struct Cursor *view = &cursor;
    struct Point *last = &view->points[2];
    union Number *selected = &view->numbers[1];

    last->x = 9;
    selected->value += 6;
    return points[2].x + numbers[1].value;
}
