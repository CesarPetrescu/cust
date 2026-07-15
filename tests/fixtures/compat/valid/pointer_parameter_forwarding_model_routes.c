struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

int int_left[6] = {11, 12, 13, 14, 15, 16};
static int int_right[6] = {21, 22, 23, 24, 25, 26};
const int const_int_left[6] = {31, 32, 33, 34, 35, 36};
static const int const_int_right[6] = {41, 42, 43, 44, 45, 46};
struct Point point_left[6] = {{51}, {52}, {53}, {54}, {55}, {56}};
static struct Point point_right[6] = {{61}, {62}, {63}, {64}, {65}, {66}};
const struct Point const_point_left[6] = {{71}, {72}, {73}, {74}, {75}, {76}};
static const struct Point const_point_right[6] = {{81}, {82}, {83}, {84}, {85}, {86}};
union Number number_left[6] = {{91}, {92}, {93}, {94}, {95}, {96}};
static union Number number_right[6] = {{101}, {102}, {103}, {104}, {105}, {106}};
const union Number const_number_left[6] = {{111}, {112}, {113}, {114}, {115}, {116}};
static const union Number const_number_right[6] = {{121}, {122}, {123}, {124}, {125}, {126}};

int *forward_int(int *value) {
    return value;
}

int *forward_int_twice(int *value) {
    return forward_int(value);
}

const int *forward_const_int(const int *value) {
    return value;
}

const int *forward_const_int_twice(const int *value) {
    return forward_const_int(value);
}

struct Point *forward_point(struct Point *value) {
    return value;
}

struct Point *forward_point_twice(struct Point *value) {
    return forward_point(value);
}

const struct Point *forward_const_point(const struct Point *value) {
    return value;
}

const struct Point *forward_const_point_twice(const struct Point *value) {
    return forward_const_point(value);
}

union Number *forward_number(union Number *value) {
    return value;
}

union Number *forward_number_twice(union Number *value) {
    return forward_number(value);
}

const union Number *forward_const_number(const union Number *value) {
    return value;
}

const union Number *forward_const_number_twice(const union Number *value) {
    return forward_const_number(value);
}

int main(void) {
    int *i0 = forward_int_twice(int_left + 1);
    int *i1 = &forward_int(int_right + 2)[0];
    const int *ci = forward_const_int_twice(const_int_left + 3);
    const int *ci_right = forward_const_int(const_int_right + 1);
    const int *promoted = forward_const_int(int_left + 4);
    struct Point *p0 = forward_point(point_left + 1);
    struct Point *p1 = &forward_point_twice(point_right + 2)[0];
    const struct Point *cp = forward_const_point(const_point_left + 3);
    const struct Point *cp_right =
        forward_const_point_twice(const_point_right + 2);
    union Number *n0 = forward_number_twice(number_right + 1);
    const union Number *cn = forward_const_number_twice(const_number_right + 2);

    int differences = i1 - int_right;
    differences += p1 - point_right;
    differences += cn - const_number_right;
    int comparisons = i0 == int_left + 1;
    comparisons += p1 > point_right;
    comparisons += ci_right > const_int_right;
    comparisons += cp_right >= const_point_right + 2;
    comparisons += cn >= const_number_right + 2;

    int *selected = 0 ? forward_int(int_left + 4)
                      : forward_int_twice(int_right + 3);
    int marker = 0;
    struct Point *comma =
        (marker += forward_point(point_left + 4) -
                   forward_point_twice(point_left + 1),
         forward_point(point_left));

    i1[0] = 7;
    p1->value = 8;
    n0->value = 9;

    return i0[0] + i1[0] + (*ci - 30) + (*promoted - 10) + p0->value +
           p1->value + (cp->value - 70) + n0->value + (cn->value - 120) +
           differences + comparisons + (selected - int_right) +
           (comma - point_left) + marker;
}
