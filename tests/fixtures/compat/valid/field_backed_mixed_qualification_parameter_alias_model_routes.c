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

int observe_int_fields(int *writer, const int *reader, int *writer_fallback,
                       const int *reader_fallback, int replacement, int expected) {
    *writer = replacement;
    int observed = *reader;
    writer = writer_fallback;
    reader = reader_fallback;
    return (observed == expected) + (writer == writer_fallback) +
           (reader == reader_fallback);
}

int observe_char_fields(char *writer, const char *reader, char *writer_fallback,
                        const char *reader_fallback, int replacement, int expected) {
    *writer = replacement;
    int observed = *reader;
    writer = writer_fallback;
    reader = reader_fallback;
    return (observed == expected) + (writer == writer_fallback) +
           (reader == reader_fallback);
}

int observe_point_fields(struct Point *writer, const struct Point *reader,
                         struct Point *writer_fallback,
                         const struct Point *reader_fallback, int replacement,
                         int expected) {
    writer->value = replacement;
    int observed = reader->value;
    writer = writer_fallback;
    reader = reader_fallback;
    return (observed == expected) + (writer == writer_fallback) +
           (reader == reader_fallback);
}

int observe_number_fields(union Number *writer, const union Number *reader,
                          union Number *writer_fallback,
                          const union Number *reader_fallback, int replacement,
                          int expected) {
    writer->value = replacement;
    int observed = reader->value;
    writer = writer_fallback;
    reader = reader_fallback;
    return (observed == expected) + (writer == writer_fallback) +
           (reader == reader_fallback);
}

int main(void) {
    struct IntFieldHolder ints = {
        .primary = {1, 2, 3, 4},
        .secondary = {5, 6, 7, 8},
    };
    struct IntFieldHolder *ints_view = &ints;
    int *int_writer = ints.primary + 1;
    const int *int_reader = ints_view->primary + 1;
    int checks = observe_int_fields(int_writer, int_reader, ints.secondary + 3,
                                    ints.primary + 2, 10, 10);
    checks += (int_writer == ints.primary + 1) +
              (int_reader == ints.primary + 1) + (ints.primary[1] == 10) +
              (ints.primary[0] == 1);

    struct CharFieldHolder chars = {
        .primary = {1, 2, 3, 4},
        .secondary = {5, 6, 7, 8},
    };
    struct CharFieldHolder *chars_view = &chars;
    char *char_writer = chars_view->primary;
    const char *char_reader = chars.primary + 2;
    checks += observe_char_fields(char_writer, char_reader, chars.secondary + 3,
                                  chars.primary + 1, 11, 3);
    checks += (char_writer == chars.primary) +
              (char_reader == chars.primary + 2) + (chars.primary[0] == 11) +
              (chars.primary[2] == 3);

    struct PointFieldHolder points = {
        .primary = {{1}, {2}, {3}, {4}},
        .secondary = {{5}, {6}, {7}, {8}},
    };
    struct PointFieldHolder *points_view = &points;
    struct Point *point_writer = points.primary + 2;
    const struct Point *point_reader = points_view->secondary + 2;
    checks += observe_point_fields(point_writer, point_reader, points.secondary + 3,
                                   points.primary + 1, 12, 7);
    checks += (point_writer == points.primary + 2) +
              (point_reader == points.secondary + 2) +
              (points.primary[2].value == 12) +
              (points.secondary[2].value == 7);

    struct NumberFieldHolder left_numbers = {
        .primary = {{1}, {2}, {3}, {4}},
        .secondary = {{5}, {6}, {7}, {8}},
    };
    struct NumberFieldHolder right_numbers = {
        .primary = {{9}, {10}, {11}, {12}},
        .secondary = {{13}, {14}, {15}, {16}},
    };
    struct NumberFieldHolder *right_numbers_view = &right_numbers;
    union Number *number_writer = right_numbers_view->primary + 1;
    const union Number *number_reader = left_numbers.primary + 1;
    checks += observe_number_fields(number_writer, number_reader,
                                    right_numbers.secondary + 3,
                                    left_numbers.primary + 2, 13, 2);
    checks += (number_writer == right_numbers.primary + 1) +
              (number_reader == left_numbers.primary + 1) +
              (right_numbers.primary[1].value == 13) +
              (left_numbers.primary[1].value == 2);

    return checks;
}
