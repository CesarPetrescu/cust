struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int bump_points(struct Point *points, int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += points[i].x + points[i].y;
        points[i].x += i + 1;
        points[i].y += 2;
        i++;
    }
    return total;
}

int sum_numbers(union Number *numbers, int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += numbers[i].value;
        numbers[i].tag += 1;
        total += numbers[i].value;
        i++;
    }
    return total;
}

int first_const_x(const struct Point *points) {
    return points[0].x;
}

int main() {
    struct Point points[2] = {{1, 2}, {3, 4}};
    union Number numbers[2] = {{5}, {7}};

    int before = bump_points(points, 2);
    int after = points[0].x + points[0].y + points[1].x + points[1].y;
    int union_total = sum_numbers(numbers, 2);
    int const_view = first_const_x(points);

    return before + after + union_total + const_view;
}
