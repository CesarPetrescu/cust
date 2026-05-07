struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int sum_points(struct Point *points) {
    points[1].x += 5;
    ++points[2].y;
    return points[0].x + points[1].x + points[2].y;
}

int sum_numbers(union Number *numbers) {
    numbers[0].value = numbers[1].value + 2;
    numbers[2].tag++;
    return numbers[0].tag + numbers[2].value;
}

int main(void) {
    struct Point points[3] = {{1, 2}, {3, 4}, {5, 6}};
    union Number numbers[3] = {{7}, {8}, {9}};

    struct Point *p = &points[0];
    union Number *n = &numbers[0];

    int total = 0;
    total += sum_points(p);
    total += points[1].x;
    total += sum_numbers(n);
    total += numbers[2].tag;

    p[0].x = p[1].y = 10;
    total += p[0].x + points[1].y;

    n[1].value += p[2].x;
    total += n[1].tag;

    return total;
}
