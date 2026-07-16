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

int observe_int_pair(int *writer, const int *reader, int replacement, int expected) {
    *writer = replacement;
    int observed = *reader;
    writer = int_right + 3;
    reader = int_left + 2;
    return (observed == expected) + (writer == int_right + 3) + (reader == int_left + 2);
}

int observe_point_pair(struct Point *writer, const struct Point *reader, int replacement,
                       int expected) {
    writer->value = replacement;
    int observed = reader->value;
    writer = point_right + 3;
    reader = point_left + 2;
    return (observed == expected) + (writer == point_right + 3) +
           (reader == point_left + 2);
}

int observe_number_pair(union Number *writer, const union Number *reader, int replacement,
                        int expected) {
    writer->value = replacement;
    int observed = reader->value;
    writer = number_right + 3;
    reader = number_left + 2;
    return (observed == expected) + (writer == number_right + 3) +
           (reader == number_left + 2);
}

int run_int_cases(void) {
    int *same_writer = int_left + 1;
    const int *same_reader = int_left + 1;
    int *distinct_writer = int_left + 2;
    const int *distinct_reader = int_left + 3;
    int *cross_writer = int_right;
    const int *cross_reader = int_left;
    int checks = observe_int_pair(same_writer, same_reader, 10, 10);
    checks += observe_int_pair(distinct_writer, distinct_reader, 11, 4);
    checks += observe_int_pair(cross_writer, cross_reader, 12, 1);
    return checks + (same_writer == int_left + 1) + (same_reader == int_left + 1) +
           (distinct_writer == int_left + 2) + (distinct_reader == int_left + 3) +
           (cross_writer == int_right) + (cross_reader == int_left) +
           (int_left[0] == 1) + (int_left[1] == 10) + (int_left[2] == 11) +
           (int_left[3] == 4) + (int_right[0] == 12) + (int_right[3] == 8);
}

int run_point_cases(void) {
    struct Point *same_writer = point_left;
    const struct Point *same_reader = point_left;
    struct Point *distinct_writer = point_left + 1;
    const struct Point *distinct_reader = point_left + 3;
    struct Point *cross_writer = point_right + 2;
    const struct Point *cross_reader = point_left + 2;
    int checks = observe_point_pair(same_writer, same_reader, 20, 20);
    checks += observe_point_pair(distinct_writer, distinct_reader, 21, 4);
    checks += observe_point_pair(cross_writer, cross_reader, 22, 3);
    return checks + (same_writer == point_left) + (same_reader == point_left) +
           (distinct_writer == point_left + 1) + (distinct_reader == point_left + 3) +
           (cross_writer == point_right + 2) + (cross_reader == point_left + 2) +
           (point_left[0].value == 20) + (point_left[1].value == 21) +
           (point_left[2].value == 3) + (point_left[3].value == 4) +
           (point_right[2].value == 22) + (point_right[3].value == 8);
}

int run_number_cases(void) {
    union Number *same_writer = number_right + 3;
    const union Number *same_reader = number_right + 3;
    union Number *distinct_writer = number_right;
    const union Number *distinct_reader = number_right + 2;
    union Number *cross_writer = number_left + 1;
    const union Number *cross_reader = number_right + 1;
    int checks = observe_number_pair(same_writer, same_reader, 30, 30);
    checks += observe_number_pair(distinct_writer, distinct_reader, 31, 7);
    checks += observe_number_pair(cross_writer, cross_reader, 32, 6);
    return checks + (same_writer == number_right + 3) + (same_reader == number_right + 3) +
           (distinct_writer == number_right) + (distinct_reader == number_right + 2) +
           (cross_writer == number_left + 1) + (cross_reader == number_right + 1) +
           (number_left[1].value == 32) + (number_left[3].value == 4) +
           (number_right[0].value == 31) + (number_right[1].value == 6) +
           (number_right[2].value == 7) + (number_right[3].value == 30);
}

int main(void) {
    return run_int_cases() + run_point_cases() + run_number_cases();
}
