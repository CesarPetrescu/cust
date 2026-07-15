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

int *pick_int(int right, int index) {
    return (right ? int_right : int_left) + index;
}

const int *pick_const_int(int right, int index) {
    return (right ? const_int_right : const_int_left) + index;
}

struct Point *pick_point(int right, int index) {
    return (right ? point_right : point_left) + index;
}

const struct Point *pick_const_point(int right, int index) {
    return (right ? const_point_right : const_point_left) + index;
}

union Number *pick_number(int right, int index) {
    return (right ? number_right : number_left) + index;
}

const union Number *pick_const_number(int right, int index) {
    return (right ? const_number_right : const_number_left) + index;
}

int main(void) {
    int *i0 = pick_int(0, 1);
    int *i1 = &pick_int(1, 2)[0];
    const int *ci = pick_const_int(0, 3);
    struct Point *p0 = pick_point(0, 1);
    struct Point *p1 = &pick_point(1, 2)[0];
    const struct Point *cp = pick_const_point(0, 3);
    union Number *n0 = pick_number(1, 1);
    const union Number *cn = pick_const_number(1, 2);

    int differences = i1 - int_right;
    differences += p1 - point_right;
    differences += cn - const_number_right;
    int comparisons = i0 == int_left + 1;
    comparisons += p1 > point_right;
    comparisons += cn >= const_number_right + 2;

    int *selected = 0 ? pick_int(0, 4) : pick_int(1, 3);
    int marker = 0;
    struct Point *comma =
        (marker += pick_point(0, 4) - pick_point(0, 1), pick_point(0, 0));

    i1[0] = 7;
    p1->value = 8;
    n0->value = 9;

    return i0[0] + i1[0] + (*ci - 30) + p0->value + p1->value +
           (cp->value - 70) + n0->value + (cn->value - 120) + differences +
           comparisons + (selected - int_right) + (comma - point_left) + marker;
}
