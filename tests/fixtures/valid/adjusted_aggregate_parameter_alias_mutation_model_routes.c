struct Point {
    int value;
};

struct Inner {
    int values[3];
    struct Point points[2];
};

struct Item {
    struct Inner nested;
};

struct Wrapper {
    struct Item items[2];
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
    return (before == 10) + (after == 12) + (first == fallback)
        + (second == fallback) + (reader == fallback);
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
    return (before == 20) + (after == 23) + (first == fallback)
        + (second == fallback) + (reader == fallback);
}

int probe(struct Item items[]) {
    int *first = forward_int(&items[0].nested.values[1]);
    int *second = forward_int_twice(&items[0].nested.values[1]);
    const int *reader = forward_const_int(&items[0].nested.values[1]);
    int scalar_score = mutate_int(first, second, reader, first);
    scalar_score += (first == &items[0].nested.values[1]);
    scalar_score += (second == &1[items[0].nested.values]);
    scalar_score += (reader == &items[0].nested.values[1]);
    scalar_score += *reader == 12;

    struct Point *point_first = forward_point(&items[0].nested.points[1]);
    struct Point *point_second = forward_point_twice(&1[items[0].nested.points]);
    const struct Point *point_reader = forward_const_point(&items[0].nested.points[1]);
    int point_score = mutate_point(
        point_first,
        point_second,
        point_reader,
        point_first
    );
    point_score += point_first == &items[0].nested.points[1];
    point_score += point_second == &1[items[0].nested.points];
    point_score += point_reader == &items[0].nested.points[1];
    point_score += point_reader->value == 23;
    return scalar_score + point_score;
}

int main(void) {
    struct Item root[2];
    struct Wrapper wrapper;
    root[0].nested.values[1] = 4;
    root[0].nested.points[1].value = 5;
    wrapper.items[0].nested.values[1] = 6;
    wrapper.items[0].nested.points[1].value = 7;
    return probe(root) + probe(wrapper.items)
        + (root[0].nested.values[1] == 12)
        + (wrapper.items[0].nested.values[1] == 12)
        + (root[0].nested.points[1].value == 23)
        + (wrapper.items[0].nested.points[1].value == 23);
}
