struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int sum_points(struct Point points[], int len) {
    int i = 0;
    int total = 0;
    while (i < len) {
        total += points[i].x + points[i].y;
        points[i].x += 1;
        i++;
    }
    return total;
}

int sum_numbers(union Number numbers[], int len) {
    int i = 0;
    int total = 0;
    while (i < len) {
        total += numbers[i].value;
        numbers[i].value += i;
        i++;
    }
    return total;
}

int main(void) {
    struct Point points[] = {{1, 2}, {.y = 4}, [3] = {5, 6}};
    int len = sizeof(points) / sizeof(points[0]);
    int before = sum_points(points, len);
    struct Point *tail = points + 3;
    int alias = tail->x + points[1].x;

    const struct Point fixed[] = {{7, 8}, {.x = 9}};
    int fixed_len = sizeof(fixed) / sizeof(fixed[0]);
    int fixed_sum = fixed[0].x + fixed[0].y + fixed[1].x + fixed[1].y;

    union Number numbers[] = {{3}, [2] = {.value = 5}};
    int number_len = sizeof(numbers) / sizeof(numbers[0]);
    int number_sum = sum_numbers(numbers, number_len);

    return before + alias + fixed_len + fixed_sum + number_sum + numbers[2].value;
}
