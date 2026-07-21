struct Point {
    int value;
};

struct Inner {
    int values[3];
    struct Point points[3];
};

struct Item {
    int capture;
    struct Inner nested;
};

struct Layer {
    struct Item primary[3];
    struct Item secondary[3];
};

struct NamedHolder {
    struct Layer nested;
};

struct AnonymousHolder {
    struct {
        struct Item primary[3];
        struct Item secondary[3];
    } nested;
};

union Choice {
    struct Layer nested;
    int marker;
};

const int *promote_int(int *value) {
    return value;
}

const int *promote_int_twice(int *value) {
    return promote_int(value);
}

const struct Point *promote_point(struct Point *value) {
    return value;
}

const struct Point *promote_point_twice(struct Point *value) {
    return promote_point(value);
}

int read_values(struct Item items[]) {
    struct Item *original = items;
    int *raw = &items[0].nested.values[0];
    int before_selected = 0;
    int before_unselected = 0;
    int middle_comma = 0;
    int after_selected = 0;
    int after_unselected = 0;

    const int *before = (1
            ? (before_selected++, promote_int(raw))
            : (before_unselected++, promote_int(&items[0].nested.values[2])))
        + 1;
    const int *middle = promote_int_twice((middle_comma++, raw)) + 1;
    const int *after = promote_int((0
            ? (after_unselected++, &items[0].nested.values[2])
            : (after_selected++, raw))
        + 1);

    int score = (*before == 7) + (*middle == 7) + (*after == 7)
        + (before == raw + 1) + (middle == raw + 1) + (after == raw + 1)
        + (original == items);
    const int *fallback = &items[-1].nested.values[0];
    before = fallback;
    return score + (*before == 3) + (before == fallback)
        + (before_selected == 1 && before_unselected == 0
            && middle_comma == 1
            && after_selected == 1 && after_unselected == 0);
}

int read_points(struct Item items[]) {
    struct Item *original = items;
    struct Point *raw = &items[0].nested.points[0];
    int before_selected = 0;
    int before_unselected = 0;
    int middle_comma = 0;
    int after_selected = 0;
    int after_unselected = 0;

    const struct Point *before = (1
            ? (before_selected++, promote_point(raw))
            : (before_unselected++, promote_point(&items[0].nested.points[2])))
        + 1;
    const struct Point *middle = promote_point_twice((middle_comma++, raw)) + 1;
    const struct Point *after = promote_point((0
            ? (after_unselected++, &items[0].nested.points[2])
            : (after_selected++, raw))
        + 1);

    int score = (before->value == 17) + (middle->value == 17) + (after->value == 17)
        + (before == raw + 1) + (middle == raw + 1) + (after == raw + 1)
        + (original == items);
    const struct Point *fallback = &items[-1].nested.points[0];
    before = fallback;
    return score + (before->value == 13) + (before == fallback)
        + (before_selected == 1 && before_unselected == 0
            && middle_comma == 1
            && after_selected == 1 && after_unselected == 0);
}

int main(void) {
    int named_capture = 0;
    int anonymous_capture = 0;
    int choice_capture = 0;

    struct NamedHolder *named = &(struct NamedHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {3}}},
                {.capture = ++named_capture, .nested = {.values = {5, 7, 9}}},
                {},
            },
        },
    };
    struct AnonymousHolder *anonymous = &(struct AnonymousHolder){
        .nested = {
            .primary = {
                {.nested = {.points = {{13}}}},
                {.capture = ++anonymous_capture, .nested = {.points = {{11}, {17}, {19}}}},
                {},
            },
        },
    };
    union Choice *choice = &(union Choice){
        .nested = {
            .primary = {
                {.nested = {.values = {3}}},
                {.capture = ++choice_capture, .nested = {.values = {5, 7, 9}}},
                {},
            },
        },
    };

    int score = read_values(named->nested.primary + 1);
    score += read_points(anonymous->nested.primary + 1);
    score += read_values(choice->nested.primary + 1);
    return score
        + (named_capture == 1 && anonymous_capture == 1 && choice_capture == 1);
}
