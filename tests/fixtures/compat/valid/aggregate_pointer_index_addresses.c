struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int mutate_point(struct Point *point) {
    point->x += 10;
    point->y += 20;
    return point->x + point->y;
}

int mutate_number(union Number *number) {
    number->value += 30;
    return number->value;
}

int main(void) {
    struct Point points[3] = {{1, 2}, {3, 4}, {5, 6}};
    union Number numbers[3] = {{7}, {8}, {9}};
    struct Point *p = points;
    union Number *n = numbers;
    int point_index = 1;
    int number_index = 2;

    struct Point *middle = &p[point_index];
    union Number *last = &n[number_index];
    int total = mutate_point(middle) + mutate_number(last);

    middle->x += 1;
    last->value += 2;
    total += points[1].x + numbers[2].value;
    total += (middle - p) == point_index;
    total += (last - n) == number_index;

    const struct Point *view = &p[0];
    total += view->x;
    total += mutate_point(&p[0]);
    total += mutate_number(&n[1]);

    return total;
}
