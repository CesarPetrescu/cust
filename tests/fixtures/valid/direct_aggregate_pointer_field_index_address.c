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
    struct Point *middle = &cursor.points[1];
    union Number *selected = &cursor.numbers[1];

    middle->x = 8;
    selected->value += 7;
    return points[1].x + numbers[1].value;
}
