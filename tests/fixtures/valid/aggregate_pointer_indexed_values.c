struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int sum_point(struct Point point) {
    point.x += 10;
    return point.x + point.y;
}

int sum_number(union Number number) {
    number.value += 3;
    return number.value;
}

int main(void) {
    struct Point points[3] = {{1, 2}, {3, 4}, {5, 6}};
    union Number numbers[2] = {{7}, {9}};
    struct Point *p = &points[0];
    union Number *n = &numbers[0];

    struct Point copy;
    union Number picked;
    int total = 0;

    copy = p[1];
    copy.x += 20;
    total += sum_point(p[0]);
    total += points[0].x;
    total += copy.x + points[1].x;

    picked = n[1];
    picked.value += 4;
    total += picked.value + numbers[1].value;
    total += sum_number(n[0]);
    total += numbers[0].value;

    return total;
}
