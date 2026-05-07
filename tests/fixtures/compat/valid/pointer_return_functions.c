typedef int *IntPtr;

struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int *choose_int(int *left, int *right, int take_right) {
    return take_right ? right : left;
}

const int *const_view(const int *value) {
    return value;
}

char *choose_char(char *text) {
    return text + 1;
}

struct Point *choose_point(struct Point *points, int index) {
    return points + index;
}

union Number *choose_number(union Number *numbers, int index) {
    return numbers + index;
}

IntPtr alias_return(IntPtr value) {
    return value;
}

int main(void) {
    int a = 4;
    int b = 9;
    int *picked = choose_int(&a, &b, 1);
    *picked += 3;

    const int locked = 11;
    const int *view = const_view(&locked);

    char text[4] = {'a', 'b', 'c', 0};
    char *middle = choose_char(text);

    struct Point points[2] = {{1, 2}, {3, 4}};
    struct Point *point = choose_point(points, 1);
    point->x += *alias_return(&a);

    union Number numbers[2] = {{5}, {7}};
    union Number *number = choose_number(numbers, 1);
    number->value += point->y;

    return a + b + *view + middle[0] + point->x + point->y + number->value;
}
