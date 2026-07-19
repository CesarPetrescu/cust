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

typedef const struct Item ConstItems[2];

int marker = 0;
int left_marker = 0;
int right_marker = 0;

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
    struct Item items[],
    int first_inner,
    int second_inner,
    int reader_inner,
    int expected_before,
    int expected_after,
    int expected_first,
    int expected_second,
    int expected_reader
) {
    int *first = &items[0].nested.values[first_inner];
    int *second = &second_inner[items[0].nested.values];
    const int *reader = &items[0].nested.values[reader_inner];
    *first = 4;
    *second = 6;
    items[0].nested.values[reader_inner] = 8;
    int score = mutate_int(
        first,
        second,
        reader,
        first,
        expected_before,
        expected_after
    );
    score += first == &items[0].nested.values[first_inner];
    score += second == &second_inner[items[0].nested.values];
    score += reader == &items[0].nested.values[reader_inner];
    score += *first == expected_first;
    score += *second == expected_second;
    score += *reader == expected_reader;
    return score;
}

int probe_separate_int(struct Item first_items[], struct Item second_items[]) {
    int *first = &first_items[0].nested.values[0];
    int *second = &0[second_items[0].nested.values];
    const int *reader = &second_items[0].nested.values[0];
    *first = 4;
    *second = 8;
    int score = mutate_int(first, second, reader, first, 8, 11);
    score += first == &first_items[0].nested.values[0];
    score += second == &0[second_items[0].nested.values];
    score += reader == &second_items[0].nested.values[0];
    score += *first == 20;
    score += *second == 11;
    score += *reader == 11;
    return score;
}

int probe_point(
    struct Item items[],
    int first_inner,
    int second_inner,
    int reader_inner,
    int expected_before,
    int expected_after,
    int expected_first,
    int expected_second,
    int expected_reader
) {
    struct Point *first = &items[0].nested.points[first_inner];
    struct Point *second = &second_inner[items[0].nested.points];
    const struct Point *reader = &items[0].nested.points[reader_inner];
    first->value = 5;
    second->value = 7;
    items[0].nested.points[reader_inner].value = 9;
    int score = mutate_point(
        first,
        second,
        reader,
        first,
        expected_before,
        expected_after
    );
    score += first == &items[0].nested.points[first_inner];
    score += second == &second_inner[items[0].nested.points];
    score += reader == &items[0].nested.points[reader_inner];
    score += first->value == expected_first;
    score += second->value == expected_second;
    score += reader->value == expected_reader;
    return score;
}

int read_const(const struct Item items[]) {
    return items[0].nested.values[1] + items[1].nested.points[2].value;
}

int main(void) {
    int score = probe_int(
        (struct Item[]){{.nested = {.values = {0, 0, ++marker}}}},
        1, 1, 1,
        20, 23, 23, 23, 23
    );
    score += probe_int(
        (struct Item[]){{.nested = {.values = {0, 0, ++marker}}}},
        0, 1, 1,
        8, 11, 20, 11, 11
    );
    score += probe_separate_int(
        (struct Item[]){{.nested = {.values = {0, 0, ++left_marker}}}},
        (struct Item[]){{.nested = {.values = {0, 0, ++right_marker}}}}
    );
    score += probe_point(
        (struct Item[]){{.nested = {.values = {0, 0, ++marker}}}},
        2, 2, 2,
        30, 34, 34, 34, 34
    );
    score += probe_point(
        (struct Item[]){{.nested = {.values = {0, 0, ++marker}}}},
        0, 1, 1,
        9, 13, 30, 13, 13
    );
    score += read_const((ConstItems){
        {.nested = {.values = {3, 7, 0}}},
        {.nested = {.points = {[2] = {9}}}},
    });
    return score + (marker == 4 && left_marker == 1 && right_marker == 1);
}
