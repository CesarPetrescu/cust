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

struct NamedLayer {
    struct Item primary[2];
    struct Item secondary[2];
};

struct NamedWrapper {
    struct NamedLayer nested;
};

struct AnonymousWrapper {
    struct {
        struct Item primary[2];
        struct Item secondary[2];
    } nested;
};

union Choice {
    struct NamedLayer nested;
    int marker;
};

int *forward_int(int *slot) {
    return slot;
}

int *forward_int_twice(int *slot) {
    return forward_int(slot);
}

const int *forward_const_int(const int *slot) {
    return slot;
}

struct Point *forward_point(struct Point *slot) {
    return slot;
}

struct Point *forward_point_twice(struct Point *slot) {
    return forward_point(slot);
}

const struct Point *forward_const_point(const struct Point *slot) {
    return slot;
}

int mutate_int(int *first, int *second, const int *reader, int *fallback) {
    *first = 10;
    int before = *reader;
    *second += 2;
    int after = *reader;
    first = fallback;
    second = fallback;
    reader = fallback;
    return before * 3 + after * 5
        + (first == fallback) + (second == fallback) + (reader == fallback);
}

int mutate_point(
    struct Point *first,
    struct Point *second,
    const struct Point *reader,
    struct Point *fallback
) {
    first->value = 20;
    int before = reader->value;
    second->value += 3;
    int after = reader->value;
    first = fallback;
    second = fallback;
    reader = fallback;
    return before * 3 + after * 5
        + (first == fallback) + (second == fallback) + (reader == fallback);
}

int probe_int_same(struct Item items[]) {
    int *first = forward_int(&items[0].nested.values[1]);
    int *second = forward_int_twice(&1[items[0].nested.values]);
    const int *reader = forward_const_int(&items[0].nested.values[1]);
    *first = 4;
    int score = mutate_int(first, second, reader, first) == 93;
    score += first == &items[0].nested.values[1];
    score += second == &1[items[0].nested.values];
    score += reader == &items[0].nested.values[1];
    score += *first == 12;
    score += *second == 12;
    score += *reader == 12;
    return score;
}

int probe_int_isolated(
    struct Item first_items[],
    struct Item second_items[],
    struct Item reader_items[]
) {
    int *first = forward_int(&first_items[0].nested.values[1]);
    int *second = forward_int_twice(&1[second_items[0].nested.values]);
    const int *reader = forward_const_int(&reader_items[0].nested.values[1]);
    *first = 4;
    *second = 6;
    reader_items[0].nested.values[1] = 8;
    int score = mutate_int(first, second, reader, first) == 67;
    score += first == &first_items[0].nested.values[1];
    score += second == &1[second_items[0].nested.values];
    score += reader == &reader_items[0].nested.values[1];
    score += *first == 10;
    score += *second == 8;
    score += *reader == 8;
    return score;
}

int probe_point_same(struct Item items[]) {
    struct Point *first = forward_point(&items[0].nested.points[1]);
    struct Point *second = forward_point_twice(&1[items[0].nested.points]);
    const struct Point *reader = forward_const_point(&items[0].nested.points[1]);
    first->value = 5;
    int score = mutate_point(first, second, reader, first) == 178;
    score += first == &items[0].nested.points[1];
    score += second == &1[items[0].nested.points];
    score += reader == &items[0].nested.points[1];
    score += first->value == 23;
    score += second->value == 23;
    score += reader->value == 23;
    return score;
}

int probe_point_isolated(
    struct Item first_items[],
    struct Item second_items[],
    struct Item reader_items[]
) {
    struct Point *first = forward_point(&first_items[0].nested.points[1]);
    struct Point *second = forward_point_twice(&1[second_items[0].nested.points]);
    const struct Point *reader = forward_const_point(&reader_items[0].nested.points[1]);
    first->value = 5;
    second->value = 7;
    reader_items[0].nested.points[1].value = 9;
    int score = mutate_point(first, second, reader, first) == 75;
    score += first == &first_items[0].nested.points[1];
    score += second == &1[second_items[0].nested.points];
    score += reader == &reader_items[0].nested.points[1];
    score += first->value == 20;
    score += second->value == 10;
    score += reader->value == 9;
    return score;
}

int main(void) {
    struct Item root[2];
    struct NamedWrapper named;
    struct AnonymousWrapper anonymous;
    union Choice choice;

    int score = probe_int_same(root);
    score += probe_point_same(named.nested.primary);
    score += probe_int_isolated(
        named.nested.secondary,
        anonymous.nested.primary,
        choice.nested.primary
    );
    score += probe_point_isolated(
        choice.nested.secondary,
        root,
        anonymous.nested.secondary
    );
    score += root[0].nested.values[1] == 12;
    score += named.nested.primary[0].nested.points[1].value == 23;
    score += anonymous.nested.primary[0].nested.values[1] == 8;
    score += choice.nested.secondary[0].nested.points[1].value == 20;
    return score;
}
