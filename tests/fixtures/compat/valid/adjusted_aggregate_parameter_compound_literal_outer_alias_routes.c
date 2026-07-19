struct Point {
    int value;
};

struct Inner {
    int values[3];
    struct Point points[3];
};

struct Item {
    struct Inner nested;
};

struct Layer {
    struct Item primary[2];
    struct Item secondary[2];
};

struct NamedHolder {
    struct Layer nested;
};

struct AnonymousHolder {
    struct {
        struct Item primary[2];
        struct Item secondary[2];
    } nested;
};

union Choice {
    struct Layer nested;
    int marker;
};

int mutate_int(
    int *first,
    int *second,
    const int *reader,
    int *fallback,
    int expected_before,
    int expected_after
) {
    *first = 20;
    int before = *reader;
    *second += 3;
    int after = *reader;
    first = fallback;
    second = fallback;
    reader = fallback;
    return (before == expected_before) + (after == expected_after)
        + (first == fallback) + (second == fallback) + (reader == fallback);
}

int mutate_point(
    struct Point *first,
    struct Point *second,
    const struct Point *reader,
    struct Point *fallback,
    int expected_before,
    int expected_after
) {
    first->value = 30;
    int before = reader->value;
    second->value += 4;
    int after = reader->value;
    first = fallback;
    second = fallback;
    reader = fallback;
    return (before == expected_before) + (after == expected_after)
        + (first == fallback) + (second == fallback) + (reader == fallback);
}

int probe_int(
    struct Item first_items[],
    struct Item second_items[],
    struct Item reader_items[],
    int first_outer,
    int first_inner,
    int second_outer,
    int second_inner,
    int reader_outer,
    int reader_inner,
    int expected_before,
    int expected_after,
    int expected_first,
    int expected_second,
    int expected_reader
) {
    int *first = &first_items[first_outer].nested.values[first_inner];
    int *second = &second_inner[second_items[second_outer].nested.values];
    const int *reader = &reader_items[reader_outer].nested.values[reader_inner];
    *first = 4;
    *second = 6;
    reader_items[reader_outer].nested.values[reader_inner] = 8;
    int score = mutate_int(
        first,
        second,
        reader,
        first,
        expected_before,
        expected_after
    );
    score += first == &first_items[first_outer].nested.values[first_inner];
    score += second == &second_inner[second_items[second_outer].nested.values];
    score += reader == &reader_items[reader_outer].nested.values[reader_inner];
    score += *first == expected_first;
    score += *second == expected_second;
    score += *reader == expected_reader;
    return score;
}

int probe_point(
    struct Item first_items[],
    struct Item second_items[],
    struct Item reader_items[],
    int first_outer,
    int first_inner,
    int second_outer,
    int second_inner,
    int reader_outer,
    int reader_inner,
    int expected_before,
    int expected_after,
    int expected_first,
    int expected_second,
    int expected_reader
) {
    struct Point *first = &first_items[first_outer].nested.points[first_inner];
    struct Point *second = &second_inner[second_items[second_outer].nested.points];
    const struct Point *reader = &reader_items[reader_outer].nested.points[reader_inner];
    first->value = 5;
    second->value = 7;
    reader_items[reader_outer].nested.points[reader_inner].value = 9;
    int score = mutate_point(
        first,
        second,
        reader,
        first,
        expected_before,
        expected_after
    );
    score += first == &first_items[first_outer].nested.points[first_inner];
    score += second == &second_inner[second_items[second_outer].nested.points];
    score += reader == &reader_items[reader_outer].nested.points[reader_inner];
    score += first->value == expected_first;
    score += second->value == expected_second;
    score += reader->value == expected_reader;
    return score;
}

int main(void) {
    int marker = 0;
    struct NamedHolder *named = &(struct NamedHolder){
        .nested = {
            .primary = {{.nested = {.values = {++marker}}}},
        },
    };
    struct AnonymousHolder *anonymous = &(struct AnonymousHolder){
        .nested = {
            .primary = {{.nested = {.values = {++marker}}}},
        },
    };
    union Choice *choice = &(union Choice){
        .nested = {
            .primary = {{.nested = {.values = {++marker}}}},
        },
    };
    struct NamedHolder *separate = &(struct NamedHolder){
        .nested = {
            .primary = {{.nested = {.values = {++marker}}}},
        },
    };

    int score = probe_int(
        named->nested.primary,
        named->nested.primary,
        named->nested.primary,
        0, 1, 0, 1, 0, 1,
        20, 23, 23, 23, 23
    );
    score += probe_point(
        anonymous->nested.primary,
        anonymous->nested.primary,
        anonymous->nested.primary,
        1, 2, 1, 2, 1, 2,
        30, 34, 34, 34, 34
    );
    score += probe_int(
        choice->nested.primary,
        choice->nested.primary,
        choice->nested.primary,
        0, 0, 0, 1, 0, 1,
        8, 11, 20, 11, 11
    );
    score += probe_point(
        named->nested.primary,
        named->nested.secondary,
        named->nested.primary,
        1, 0, 1, 0, 1, 0,
        30, 30, 30, 11, 30
    );
    score += probe_int(
        named->nested.secondary,
        anonymous->nested.secondary,
        choice->nested.secondary,
        0, 2, 0, 2, 0, 2,
        8, 8, 20, 9, 8
    );
    score += probe_point(
        choice->nested.primary,
        separate->nested.primary,
        anonymous->nested.primary,
        1, 1, 1, 1, 0, 1,
        9, 9, 30, 11, 9
    );
    score += marker == 4;
    return score;
}
