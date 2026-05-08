typedef struct {
    int x;
    int y;
} Point;

typedef union {
    int value;
    char tag;
} Number;

int sum_point(Point point) {
    point.x += 1;
    return point.x + point.y;
}

Number make_number(int value) {
    Number number = {value};
    return number;
}

int main(void) {
    Point p = {3, 4};
    Point points[2] = {{1, 2}, {.y = 5, .x = 4}};
    Point *ptr = &points[1];
    Number n = make_number(6);
    Number numbers[2] = {{7}, {.tag = 8}};
    Number *np = numbers;

    ptr->x += 2;
    np[0].value += 1;
    return sum_point(p)
        + points[1].x
        + points[1].y
        + n.tag
        + numbers[0].tag
        + numbers[1].value
        + sizeof(Point)
        + sizeof(Number *);
}
