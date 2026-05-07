int sum_and_bump(int values[], int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += values[i];
        values[i] += 1;
        i++;
    }
    return total;
}

int count_chars(char text[]) {
    int i = 0;
    while (text[i] != 0) {
        i++;
    }
    return i;
}

struct Point {
    int x;
    int y;
};

union Number {
    int value;
};

int adjust_points(struct Point points[], int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += points[i].x + points[i].y;
        points[i].x += 10 + i;
        i++;
    }
    return total;
}

int bump_numbers(union Number numbers[], int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += numbers[i].value;
        numbers[i].value += 1;
        total += numbers[i].value;
        i++;
    }
    return total;
}

int first_const_x(const struct Point points[]) {
    return points[0].x;
}

int main(void) {
    int values[3] = {1, 2, 3};
    struct Point points[2] = {{2, 3}, {4, 5}};
    union Number numbers[2] = {{6}, {8}};

    int before = sum_and_bump(values, 3);
    int after = values[0] + values[1] + values[2];
    int chars = count_chars("hey");
    int point_before = adjust_points(points, 2);
    int point_after = points[0].x + points[0].y + points[1].x + points[1].y;
    int number_total = bump_numbers(numbers, 2);
    int const_view = first_const_x(points);

    return before + after + chars + point_before + point_after + number_total + const_view;
}
