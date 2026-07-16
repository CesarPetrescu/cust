struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

struct IntFields {
    int primary[4];
    int secondary[4];
};

struct NestedIntFields {
    struct IntFields inner;
};

struct PointFields {
    struct Point primary[4];
    struct Point secondary[4];
};

struct NestedPointFields {
    struct PointFields inner;
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
    struct NestedIntFields ints = {
        .inner = {
            .primary = {1, 2, 3, 4},
            .secondary = {5, 6, 7, 8},
        },
    };
    struct NestedIntFields *ints_view = &ints;
    int *int_writer = forward_int_twice(ints_view->inner.primary + 1);
    int *int_second = forward_int(ints.inner.primary + 1);
    const int *int_reader = forward_const_int_twice(ints.inner.primary + 1);
    int checks = mutate_int(int_writer, int_second, int_reader, 10, 2, 10, 12);
    checks += (int_writer == ints.inner.primary + 1) +
              (int_second == ints.inner.primary + 1) +
              (int_reader == ints.inner.primary + 1) +
              (ints.inner.primary[1] == 12) +
              (ints.inner.secondary[1] == 6);

    struct {
        char primary[4];
        char secondary[4];
    } chars = {
        .primary = {1, 2, 4, 5},
        .secondary = {6, 7, 8, 9},
    }, *chars_view = &chars;
    char *char_writer = forward_char(chars.primary);
    char *char_second = forward_char_twice(chars_view->primary + 2);
    const char *char_reader = forward_const_char(chars.primary + 2);
    checks += mutate_char(char_writer, char_second, char_reader, 11, 3, 4, 7);
    checks += (char_writer == chars.primary) +
              (char_second == chars.primary + 2) +
              (char_reader == chars.primary + 2) + (chars.primary[0] == 11) +
              (chars.primary[2] == 7);

    struct NestedPointFields points = {
        .inner = {
            .primary = {{1}, {2}, {3}, {4}},
            .secondary = {{5}, {6}, {7}, {8}},
        },
    };
    struct NestedPointFields *points_view = &points;
    struct Point *point_writer =
        forward_point_twice(points_view->inner.primary + 2);
    struct Point *point_second = forward_point(points.inner.secondary + 2);
    const struct Point *point_reader =
        forward_const_point_twice(points_view->inner.primary + 2);
    checks += mutate_point(point_writer, point_second, point_reader, 13, 4, 13, 13);
    checks += (point_writer == points.inner.primary + 2) +
              (point_second == points.inner.secondary + 2) +
              (point_reader == points.inner.primary + 2) +
              (points.inner.primary[2].value == 13) +
              (points.inner.secondary[2].value == 11);

    struct {
        union Number primary[4];
        union Number secondary[4];
    } numbers = {
        .primary = {{1}, {2}, {3}, {4}},
        .secondary = {{5}, {6}, {7}, {8}},
    }, *numbers_view = &numbers;
    union Number *number_writer =
        forward_number(numbers_view->secondary + 1);
    union Number *number_second = forward_number_twice(numbers.primary + 1);
    const union Number *number_reader =
        forward_const_number(numbers.primary + 1);
    checks += mutate_number(number_writer, number_second, number_reader, 14, 5, 2, 7);
    checks += (number_writer == numbers.secondary + 1) +
              (number_second == numbers.primary + 1) +
              (number_reader == numbers.primary + 1) +
              (numbers.secondary[1].value == 14) +
              (numbers.primary[1].value == 7);

    return checks;
}
