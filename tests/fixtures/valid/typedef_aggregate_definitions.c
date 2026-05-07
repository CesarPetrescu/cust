typedef struct Point {
    int x;
    int y;
} Point;

typedef union Number {
    int value;
    char tag;
} Number;

int sum(Point p) {
    return p.x + p.y;
}

Number make_number(int value) {
    Number n = {.tag = 0};
    n.value = value;
    return n;
}

int main(void) {
    Point origin = {.y = 5, .x = 4};
    Point points[2] = {{1, 2}, {.y = 7, .x = 6}};
    Point *cursor = points;
    cursor[0].x += origin.x;

    Number n = make_number(sum(origin));
    Number values[2] = {[1] = {.tag = 3}, [0] = {2}};
    values[1].value = values[1].value + n.value;

    return sum(cursor[0]) + sum(cursor[1]) + values[0].value + values[1].tag;
}
