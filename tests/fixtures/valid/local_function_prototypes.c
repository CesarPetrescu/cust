int add(int left, int right) {
    return left + right;
}

char pick_char(char first, char second) {
    return second - first;
}

struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

struct Point make_point(int x, int y) {
    struct Point point = {x, y};
    return point;
}

union Number make_number(int value) {
    union Number number = {value};
    return number;
}

int sum_points(struct Point points[], int count) {
    int total = 0;
    int i = 0;
    while (i < count) {
        total += points[i].x + points[i].y;
        i++;
    }
    return total;
}

int mutate(int values[], int count) {
    int index = 0;
    while (index < count) {
        values[index] += index;
        index++;
    }
    return values[0] + values[1] + values[2];
}

int main(void) {
    int add(int, int);
    extern char pick_char(char first, char second);
    extern struct Point make_point(int x, int y);
    union Number make_number(int value);
    extern int sum_points(struct Point points[], int count);
    int mutate(int values[static 3], int count);

    struct Point made = make_point(1, 2);
    struct Point points[2] = {{2, 3}, {4, 5}};
    union Number number = make_number(7);
    int values[3] = {1, 2, 3};

    return add(10, 20)
        + pick_char(3, 8)
        + made.x + made.y
        + sum_points(points, 2)
        + number.value
        + mutate(values, 3);
}
