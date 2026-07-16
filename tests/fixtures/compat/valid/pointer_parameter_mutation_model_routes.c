struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

int int_left[6] = {11, 12, 13, 14, 15, 16};
static int int_right[6] = {21, 22, 23, 24, 25, 26};
struct Point point_left[6] = {{31}, {32}, {33}, {34}, {35}, {36}};
static struct Point point_right[6] = {{41}, {42}, {43}, {44}, {45}, {46}};
union Number number_left[6] = {{51}, {52}, {53}, {54}, {55}, {56}};
static union Number number_right[6] = {{61}, {62}, {63}, {64}, {65}, {66}};

int mutate_int(int *value, int replacement) {
    *value = replacement;
    value = int_right + 5;
    return value == int_right + 5;
}

int mutate_int_twice(int *value, int replacement) {
    int checks = mutate_int(value, replacement);
    value = int_left + 5;
    return checks + (value == int_left + 5);
}

int mutate_point(struct Point *value, int replacement) {
    value->value = replacement;
    value = point_right + 5;
    return value == point_right + 5;
}

int mutate_point_twice(struct Point *value, int replacement) {
    int checks = mutate_point(value, replacement);
    value = point_left + 5;
    return checks + (value == point_left + 5);
}

int mutate_number(union Number *value, int replacement) {
    value->value = replacement;
    value = number_right + 5;
    return value == number_right + 5;
}

int mutate_number_twice(union Number *value, int replacement) {
    int checks = mutate_number(value, replacement);
    value = number_left + 5;
    return checks + (value == number_left + 5);
}

int main(void) {
    int *integer = int_left + 1;
    struct Point *point = point_right + 2;
    union Number *number = number_left + 3;

    int checks = mutate_int_twice(integer, 7);
    checks += mutate_point(point, 8);
    checks += mutate_number_twice(number, 9);

    return *integer + point->value + number->value + (integer - int_left) +
           (point - point_right) + (number - number_left) + checks;
}
