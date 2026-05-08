struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int bump_points(struct Point points[], int len) {
    int i = 0;
    int total = 0;
    while (i < len) {
        points[i].x += i;
        total += points[i].x + points[i].y;
        i++;
    }
    return total;
}

int sum_numbers(union Number numbers[], int len) {
    int i = 0;
    int total = 0;
    while (i < len) {
        total += numbers[i].value;
        i++;
    }
    return total;
}

int main(void) {
    struct Point *points = (struct Point[]){{1, 2}, {.x = 3, .y = 4}};
    int first = bump_points(points, 2);
    int after = points[1].x;
    int designated = bump_points((struct Point[3]){[2] = {.x = 5, .y = 6}, [0] = {1, 2}}, 3);
    int unions = sum_numbers((union Number[]){{7}, [2] = {.value = 9}}, 3);
    return first + after + designated + unions;
}
