struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

int int_left[4] = {1, 2, 3, 4};
static int int_right[4] = {5, 6, 7, 8};
struct Point point_left[4] = {{1}, {2}, {3}, {4}};
static struct Point point_right[4] = {{5}, {6}, {7}, {8}};
union Number number_left[4] = {{1}, {2}, {3}, {4}};
static union Number number_right[4] = {{5}, {6}, {7}, {8}};

int mutate_int_pair(int *first, int *second, int replacement, int delta) {
    *first = replacement;
    *second += delta;
    first = int_right + 3;
    second = int_left + 2;
    return (first == int_right + 3) + (second == int_left + 2);
}

int mutate_point_pair(struct Point *first, struct Point *second, int replacement, int delta) {
    first->value = replacement;
    second->value += delta;
    first = point_right + 3;
    second = point_left + 2;
    return (first == point_right + 3) + (second == point_left + 2);
}

int mutate_number_pair(union Number *first, union Number *second, int replacement, int delta) {
    first->value = replacement;
    second->value += delta;
    first = number_right + 3;
    second = number_left + 2;
    return (first == number_right + 3) + (second == number_left + 2);
}

int run_int_cases(void) {
    int *same_first = int_left + 1;
    int *same_second = int_left + 1;
    int *distinct_first = int_left + 2;
    int *distinct_second = int_left + 3;
    int *cross_first = int_right;
    int *cross_second = int_left;
    int checks = mutate_int_pair(same_first, same_second, 10, 3);
    checks += mutate_int_pair(distinct_first, distinct_second, 11, 4);
    checks += mutate_int_pair(cross_first, cross_second, 12, 5);
    return checks + (same_first == int_left + 1) + (same_second == int_left + 1) +
           (distinct_first == int_left + 2) + (distinct_second == int_left + 3) +
           (cross_first == int_right) + (cross_second == int_left) +
           (int_left[0] == 6) + (int_left[1] == 13) + (int_left[2] == 11) +
           (int_left[3] == 8) + (int_right[0] == 12) + (int_right[3] == 8);
}

int run_point_cases(void) {
    struct Point *same_first = point_left;
    struct Point *same_second = point_left;
    struct Point *distinct_first = point_left + 1;
    struct Point *distinct_second = point_left + 3;
    struct Point *cross_first = point_right + 2;
    struct Point *cross_second = point_left + 2;
    int checks = mutate_point_pair(same_first, same_second, 20, 2);
    checks += mutate_point_pair(distinct_first, distinct_second, 21, 3);
    checks += mutate_point_pair(cross_first, cross_second, 22, 4);
    return checks + (same_first == point_left) + (same_second == point_left) +
           (distinct_first == point_left + 1) + (distinct_second == point_left + 3) +
           (cross_first == point_right + 2) + (cross_second == point_left + 2) +
           (point_left[0].value == 22) + (point_left[1].value == 21) +
           (point_left[2].value == 7) + (point_left[3].value == 7) +
           (point_right[2].value == 22) + (point_right[3].value == 8);
}

int run_number_cases(void) {
    union Number *same_first = number_right + 3;
    union Number *same_second = number_right + 3;
    union Number *distinct_first = number_right;
    union Number *distinct_second = number_right + 2;
    union Number *cross_first = number_left + 1;
    union Number *cross_second = number_right + 1;
    int checks = mutate_number_pair(same_first, same_second, 30, 1);
    checks += mutate_number_pair(distinct_first, distinct_second, 31, 2);
    checks += mutate_number_pair(cross_first, cross_second, 32, 3);
    return checks + (same_first == number_right + 3) + (same_second == number_right + 3) +
           (distinct_first == number_right) + (distinct_second == number_right + 2) +
           (cross_first == number_left + 1) + (cross_second == number_right + 1) +
           (number_left[1].value == 32) + (number_left[3].value == 4) +
           (number_right[0].value == 31) + (number_right[1].value == 9) +
           (number_right[2].value == 9) + (number_right[3].value == 31);
}

int main(void) {
    return run_int_cases() + run_point_cases() + run_number_cases();
}
