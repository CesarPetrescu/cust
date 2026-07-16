struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

struct IntFieldHolder {
    int primary[4];
    int secondary[4];
};

struct CharFieldHolder {
    char primary[4];
    char secondary[4];
};

struct PointFieldHolder {
    struct Point primary[4];
    struct Point secondary[4];
};

struct NumberFieldHolder {
    union Number primary[4];
    union Number secondary[4];
};

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

char *forward_char(char *value) {
    return value;
}

char *forward_char_twice(char *value) {
    return forward_char(value);
}

const char *forward_const_char(const char *value) {
    return value;
}

const char *forward_const_char_twice(const char *value) {
    return forward_const_char(value);
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

int mutate_int(int *writer, int *second_writer, const int *reader,
               int replacement, int delta, int first_expected,
               int second_expected) {
    *writer = replacement;
    int first = *reader;
    *second_writer += delta;
    int second = *reader;
    writer = second_writer;
    reader = writer;
    second_writer = 0;
    return (first == first_expected) + (second == second_expected) +
           (writer == reader) + (second_writer == 0);
}

int mutate_char(char *writer, char *second_writer, const char *reader,
                int replacement, int delta, int first_expected,
                int second_expected) {
    *writer = replacement;
    int first = *reader;
    *second_writer += delta;
    int second = *reader;
    writer = second_writer;
    reader = writer;
    second_writer = 0;
    return (first == first_expected) + (second == second_expected) +
           (writer == reader) + (second_writer == 0);
}

int mutate_point(struct Point *writer, struct Point *second_writer,
                 const struct Point *reader, int replacement, int delta,
                 int first_expected, int second_expected) {
    writer->value = replacement;
    int first = reader->value;
    second_writer->value += delta;
    int second = reader->value;
    writer = second_writer;
    reader = writer;
    second_writer = 0;
    return (first == first_expected) + (second == second_expected) +
           (writer == reader) + (second_writer == 0);
}

int mutate_number(union Number *writer, union Number *second_writer,
                  const union Number *reader, int replacement, int delta,
                  int first_expected, int second_expected) {
    writer->value = replacement;
    int first = reader->value;
    second_writer->value += delta;
    int second = reader->value;
    writer = second_writer;
    reader = writer;
    second_writer = 0;
    return (first == first_expected) + (second == second_expected) +
           (writer == reader) + (second_writer == 0);
}

int main(void) {
    struct IntFieldHolder ints = {
        .primary = {1, 2, 3, 4},
        .secondary = {5, 6, 7, 8},
    };
    struct IntFieldHolder *ints_view = &ints;
    int *int_writer = forward_int_twice(ints_view->primary + 1);
    int *int_second = forward_int(ints.primary + 1);
    const int *int_reader = forward_const_int_twice(ints_view->primary + 1);
    int checks = mutate_int(int_writer, int_second, int_reader, 10, 2, 10, 12);
    checks += (int_writer == ints.primary + 1) +
              (int_second == ints.primary + 1) +
              (int_reader == ints.primary + 1) + (ints.primary[1] == 12) +
              (ints.secondary[1] == 6);

    struct CharFieldHolder chars = {
        .primary = {1, 2, 4, 5},
        .secondary = {6, 7, 8, 9},
    };
    struct CharFieldHolder *chars_view = &chars;
    char *char_writer = forward_char(chars.primary);
    char *char_second = forward_char_twice(chars_view->primary + 2);
    const char *char_reader = forward_const_char(chars.primary + 2);
    checks += mutate_char(char_writer, char_second, char_reader, 11, 3, 4, 7);
    checks += (char_writer == chars.primary) +
              (char_second == chars.primary + 2) +
              (char_reader == chars.primary + 2) + (chars.primary[0] == 11) +
              (chars.primary[2] == 7);

    struct PointFieldHolder points = {
        .primary = {{1}, {2}, {3}, {4}},
        .secondary = {{5}, {6}, {7}, {8}},
    };
    struct PointFieldHolder *points_view = &points;
    struct Point *point_writer = forward_point_twice(points_view->primary + 2);
    struct Point *point_second = forward_point(points.secondary + 2);
    const struct Point *point_reader =
        forward_const_point_twice(points_view->primary + 2);
    checks += mutate_point(point_writer, point_second, point_reader, 13, 4, 13, 13);
    checks += (point_writer == points.primary + 2) +
              (point_second == points.secondary + 2) +
              (point_reader == points.primary + 2) +
              (points.primary[2].value == 13) +
              (points.secondary[2].value == 11);

    struct NumberFieldHolder left_numbers = {
        .primary = {{1}, {2}, {3}, {4}},
        .secondary = {{5}, {6}, {7}, {8}},
    };
    struct NumberFieldHolder right_numbers = {
        .primary = {{9}, {10}, {11}, {12}},
        .secondary = {{13}, {14}, {15}, {16}},
    };
    struct NumberFieldHolder *right_numbers_view = &right_numbers;
    union Number *number_writer =
        forward_number(right_numbers_view->primary + 1);
    union Number *number_second = forward_number_twice(left_numbers.primary + 1);
    const union Number *number_reader =
        forward_const_number(left_numbers.primary + 1);
    checks += mutate_number(number_writer, number_second, number_reader, 14, 5, 2, 7);
    checks += (number_writer == right_numbers.primary + 1) +
              (number_second == left_numbers.primary + 1) +
              (number_reader == left_numbers.primary + 1) +
              (right_numbers.primary[1].value == 14) +
              (left_numbers.primary[1].value == 7);

    return checks;
}
