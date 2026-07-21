struct Point {
    int value;
};

struct Inner {
    int values[4];
    struct Point points[4];
};

struct Item {
    int capture;
    struct Inner nested;
};

const int *promote_int(int *value) {
    return value;
}

const int *promote_int_twice(int *value) {
    return promote_int(value);
}

const int *reforward_int(const int *value) {
    return value;
}

const int *reforward_int_twice(const int *value) {
    return reforward_int(value);
}

const struct Point *promote_point(struct Point *value) {
    return value;
}

const struct Point *promote_point_twice(struct Point *value) {
    return promote_point(value);
}

const struct Point *reforward_point(const struct Point *value) {
    return value;
}

const struct Point *reforward_point_twice(const struct Point *value) {
    return reforward_point(value);
}

int read_values(struct Item items[]) {
    struct Item *outer_original = items;
    int *raw = &items[0].nested.values[0];
    int first_selected = 0;
    int first_unselected = 0;
    int first_comma = 0;
    int post_selected = 0;
    int post_unselected = 0;
    int post_comma = 0;

    const int *promoted_before = (1
            ? (first_selected++, promote_int(raw))
            : (first_unselected++, promote_int(&items[0].nested.values[3])))
        + 1;
    const int *promoted_middle = promote_int_twice((first_comma++, raw)) + 1;
    const int *promoted_after = promote_int((0
            ? (first_unselected++, &items[0].nested.values[3])
            : (first_selected++, raw))
        + 1);

    const int *before = (1
            ? (post_selected++, reforward_int(promoted_before))
            : (post_unselected++, reforward_int(&items[0].nested.values[3])))
        + 1;
    const int *middle = reforward_int_twice((post_comma++, promoted_middle)) + 1;
    const int *after = reforward_int((0
            ? (post_unselected++, &items[0].nested.values[3])
            : (post_selected++, promoted_after))
        + 1);

    int score = (*before == 9) + (*middle == 9) + (*after == 9)
        + (before == raw + 2) + (middle == raw + 2) + (after == raw + 2)
        + (*promoted_before == 7) + (*promoted_middle == 7) + (*promoted_after == 7)
        + (promoted_before == raw + 1)
        + (promoted_middle == raw + 1)
        + (promoted_after == raw + 1)
        + (outer_original == items);
    const int *slot = before;
    score += slot == before;
    struct Item *fallback_items = items - 1;
    items = fallback_items;
    slot = &items[0].nested.values[0];
    return score + (*slot == 3) + (slot == &items[0].nested.values[0])
        + (items == fallback_items)
        + (first_selected == 2 && first_unselected == 0 && first_comma == 1)
        + (post_selected == 2 && post_unselected == 0 && post_comma == 1);
}

int read_points(struct Item items[]) {
    struct Item *outer_original = items;
    struct Point *raw = &items[0].nested.points[0];
    int first_selected = 0;
    int first_unselected = 0;
    int first_comma = 0;
    int post_selected = 0;
    int post_unselected = 0;
    int post_comma = 0;

    const struct Point *promoted_before = (1
            ? (first_selected++, promote_point(raw))
            : (first_unselected++, promote_point(&items[0].nested.points[3])))
        + 1;
    const struct Point *promoted_middle = promote_point_twice((first_comma++, raw)) + 1;
    const struct Point *promoted_after = promote_point((0
            ? (first_unselected++, &items[0].nested.points[3])
            : (first_selected++, raw))
        + 1);

    const struct Point *before = (1
            ? (post_selected++, reforward_point(promoted_before))
            : (post_unselected++, reforward_point(&items[0].nested.points[3])))
        + 1;
    const struct Point *middle = reforward_point_twice((post_comma++, promoted_middle)) + 1;
    const struct Point *after = reforward_point((0
            ? (post_unselected++, &items[0].nested.points[3])
            : (post_selected++, promoted_after))
        + 1);

    int score = (before->value == 19) + (middle->value == 19) + (after->value == 19)
        + (before == raw + 2) + (middle == raw + 2) + (after == raw + 2)
        + (promoted_before->value == 17)
        + (promoted_middle->value == 17)
        + (promoted_after->value == 17)
        + (promoted_before == raw + 1)
        + (promoted_middle == raw + 1)
        + (promoted_after == raw + 1)
        + (outer_original == items);
    const struct Point *slot = before;
    score += slot == before;
    struct Item *fallback_items = items - 1;
    items = fallback_items;
    slot = &items[0].nested.points[0];
    return score + (slot->value == 13) + (slot == &items[0].nested.points[0])
        + (items == fallback_items)
        + (first_selected == 2 && first_unselected == 0 && first_comma == 1)
        + (post_selected == 2 && post_unselected == 0 && post_comma == 1);
}

int main(void) {
    int values_capture = 0;
    int points_capture = 0;

    int score = read_values(
        (struct Item[3]){
            {.nested = {.values = {3}}},
            {.capture = ++values_capture, .nested = {.values = {5, 7, 9, 11}}},
            {},
        }
        + 1
    );
    score += read_points(
        &((struct Item[3]){
            {.nested = {.points = {{13}}}},
            {.capture = ++points_capture, .nested = {.points = {{11}, {17}, {19}, {23}}}},
            {},
        })[1]
    );
    return score + (values_capture == 1 && points_capture == 1);
}
