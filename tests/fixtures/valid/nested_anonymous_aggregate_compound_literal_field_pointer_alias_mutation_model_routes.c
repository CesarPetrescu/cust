struct Point {
    int value;
};

struct IntFields {
    int primary[4];
    int secondary[4];
};

struct PointFields {
    struct Point primary[4];
    struct Point secondary[4];
};

struct NestedIntFields {
    struct IntFields inner;
};

struct AnonymousIntFields {
    struct {
        int primary[4];
        int secondary[4];
    } inner;
};

struct NestedPointFields {
    struct PointFields inner;
};

struct AnonymousPointFields {
    struct {
        struct Point primary[4];
        struct Point secondary[4];
    } inner;
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

struct Point *forward_point(struct Point *value) {
    return value;
}

struct Point *forward_point_twice(struct Point *value) {
    return forward_point(value);
}

const struct Point *forward_const_point(const struct Point *value) {
    return value;
}

int mutate_int(int *writer, int *second_writer, const int *reader) {
    *writer = 20;
    int first = *reader;
    *second_writer += 3;
    int second = *reader;
    writer = second_writer;
    reader = writer;
    second_writer = 0;
    return (first == 20) + (second == 23) + (writer == reader) +
           (second_writer == 0);
}

int mutate_point(struct Point *writer, struct Point *second_writer,
                 const struct Point *reader) {
    writer->value = 30;
    int first = reader->value;
    second_writer->value += 4;
    int second = reader->value;
    writer = second_writer;
    reader = writer;
    second_writer = 0;
    return (first == 30) + (second == 34) + (writer == reader) +
           (second_writer == 0);
}

int main(void) {
    int marker = 0;
    struct NestedIntFields *nested_int = &(struct NestedIntFields){
        .inner = {
            .primary = {++marker, 2, 3, 4},
            .secondary = {5, 6, 7, 8},
        },
    };
    struct AnonymousIntFields *anonymous_int = &(struct AnonymousIntFields){
        .inner = {
            .primary = {++marker, 10, 11, 12},
            .secondary = {13, 14, 15, 16},
        },
    };
    int *int_writer = &forward_int_twice(nested_int->inner.primary + 1)[0];
    int *int_second = forward_int(nested_int->inner.primary) + 1;
    const int *int_reader =
        (marker += 0, forward_const_int(nested_int->inner.primary + 1));

    int checks = mutate_int(int_writer, int_second, int_reader);
    checks += (int_writer == nested_int->inner.primary + 1) +
              (int_second == nested_int->inner.primary + 1) +
              (int_reader == nested_int->inner.primary + 1) +
              (nested_int->inner.primary[1] == 23) +
              (anonymous_int->inner.primary[1] == 10) + (marker == 2);

    struct NestedPointFields *nested_point = &(struct NestedPointFields){
        .inner = {
            .primary = {{++marker}, {2}, {3}, {4}},
            .secondary = {{5}, {6}, {7}, {8}},
        },
    };
    struct AnonymousPointFields *anonymous_point = &(struct AnonymousPointFields){
        .inner = {
            .primary = {{++marker}, {10}, {11}, {12}},
            .secondary = {{13}, {14}, {15}, {16}},
        },
    };
    struct Point *point_writer =
        forward_point_twice(nested_point->inner.primary + 2);
    struct Point *point_second =
        marker == 4 ? forward_point(nested_point->inner.primary + 2)
                    : anonymous_point->inner.primary;
    const struct Point *point_reader =
        &forward_const_point(nested_point->inner.primary + 2)[0];

    checks += mutate_point(point_writer, point_second, point_reader);
    checks += (point_writer == nested_point->inner.primary + 2) +
              (point_second == nested_point->inner.primary + 2) +
              (point_reader == nested_point->inner.primary + 2) +
              (nested_point->inner.primary[2].value == 34) +
              (anonymous_point->inner.primary[2].value == 11) +
              (marker == 4);

    return checks;
}
