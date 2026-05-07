struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int sum_points(struct Point *points) {
    return points[0].x + points[0].y + points[1].x + points[1].y + points[2].x + points[2].y;
}

int sum_numbers(union Number *numbers) {
    return numbers[0].value + numbers[1].tag + numbers[2].value;
}

int main(void) {
    struct Point points[3] = {[2] = {.y = 6, .x = 5}, [0] = {1, 2}, [1] = {0, 0}};
    union Number numbers[3] = {[1] = {.tag = 7}, [2] = {4}, [0] = {0}};

    int sum = sum_points(&points[0]);
    sum += sum_numbers(&numbers[0]);

    points[1].x = 3;
    points[1].y = 4;
    numbers[0].value = 8;

    return sum + points[1].x + points[1].y + numbers[0].tag;
}
