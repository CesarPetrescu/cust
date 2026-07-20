struct Point {
    int value;
};

struct Inner {
    int values[2];
    struct Point points[2];
};

struct Item {
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

struct Item *forward_items(struct Item *items) {
    return items;
}

struct Item *forward_items_twice(struct Item *items) {
    return forward_items(items);
}

const struct Item *forward_const_items(const struct Item *items) {
    return items;
}

const struct Item *forward_const_items_twice(const struct Item *items) {
    return forward_const_items(items);
}

int mutate_slots(int *scalar, struct Point *point, int *fallback, struct Point *point_fallback) {
    *scalar += 2;
    point->value += 3;
    int score = (*scalar == 7) + (point->value == 10);
    scalar = fallback;
    point = point_fallback;
    return score + (scalar == fallback) + (point == point_fallback);
}

int probe(struct Item items[]) {
    int *scalar = &items[0].nested.values[1];
    struct Point *point = &items[0].nested.points[1];
    int score = mutate_slots(scalar, point, scalar, point);
    score += scalar == &items[0].nested.values[1];
    score += point == &items[0].nested.points[1];
    score += items[-1].nested.values[0] == 9;
    score += *scalar == 7;
    score += point->value == 10;
    return score;
}

int read_const(const struct Item items[]) {
    return (items[-1].nested.values[0] == 9)
        + (items[0].nested.values[1] == 11)
        + (items[0].nested.points[1].value == 13);
}

int main(void) {
    int named_capture = 0;
    int anonymous_capture = 0;
    int choice_capture = 0;
    int locked_capture = 0;

    struct NamedHolder *named = &(struct NamedHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {named_capture++, 5}, .points = {[1] = {7}}}},
                {},
            },
            .secondary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {1, 5}, .points = {[1] = {7}}}},
                {},
            },
        },
    };
    struct AnonymousHolder *anonymous = &(struct AnonymousHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {anonymous_capture++, 5}, .points = {[1] = {7}}}},
                {},
            },
            .secondary = {
                {.nested = {.values = {1}}},
                {},
                {},
            },
        },
    };
    union Choice *choice = &(union Choice){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {choice_capture++, 5}, .points = {[1] = {7}}}},
                {},
            },
        },
    };
    const struct NamedHolder *locked = &(const struct NamedHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {locked_capture++, 11}, .points = {[1] = {13}}}},
                {},
            },
            .secondary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {1, 11}, .points = {[1] = {13}}}},
                {},
            },
        },
    };

    int named_inner_selected = 0;
    int named_inner_unselected = 0;
    int named_post_selected = 0;
    int named_post_unselected = 0;
    int anonymous_inner_selected = 0;
    int anonymous_inner_unselected = 0;
    int anonymous_post_selected = 0;
    int anonymous_post_unselected = 0;
    int choice_inner_comma = 0;
    int choice_post_comma = 0;
    int cross_inner_selected = 0;
    int cross_inner_unselected = 0;
    int cross_post_selected = 0;
    int cross_post_unselected = 0;
    int locked_inner_selected = 0;
    int locked_inner_unselected = 0;
    int locked_post_selected = 0;
    int locked_post_unselected = 0;
    int locked_inner_comma = 0;
    int locked_post_comma = 0;

    int score = probe(
        (1
             ? (named_post_selected++,
                forward_items(
                    1
                        ? (named_inner_selected++, named->nested.primary)
                        : (named_inner_unselected++, anonymous->nested.primary)))
             : (named_post_unselected++, forward_items(anonymous->nested.primary)))
        + 1
    );
    score += probe(
        0
            ? (anonymous_post_unselected++, forward_items(1 + named->nested.primary))
            : (anonymous_post_selected++,
               forward_items(
                   1
                   + (0
                          ? (anonymous_inner_unselected++, named->nested.primary)
                          : (anonymous_inner_selected++, anonymous->nested.primary))))
    );
    score += probe(
        &(choice_post_comma++,
          forward_items_twice((choice_inner_comma++, choice->nested.primary)))[1]
    );
    score += probe(
        1
            ? (cross_post_selected++,
               forward_items_twice(
                   &(1
                         ? (cross_inner_selected++, named->nested.secondary)
                         : (cross_inner_unselected++, anonymous->nested.secondary))[1]))
            : (cross_post_unselected++, forward_items_twice(&anonymous->nested.secondary[1]))
    );
    score += read_const(
        (1
             ? (locked_post_selected++,
                forward_const_items(
                    1
                        ? (locked_inner_selected++, locked->nested.primary)
                        : (locked_inner_unselected++, locked->nested.secondary)))
             : (locked_post_unselected++, forward_const_items(locked->nested.secondary)))
        + 1
    );
    score += read_const(
        (locked_post_comma++,
         forward_const_items_twice(
             &(locked_inner_comma++, locked->nested.secondary)[1]))
    );

    int captures = named_capture == 1 && anonymous_capture == 1
        && choice_capture == 1 && locked_capture == 1;
    int inner_wrappers = named_inner_selected == 1 && named_inner_unselected == 0
        && anonymous_inner_selected == 1 && anonymous_inner_unselected == 0
        && choice_inner_comma == 1
        && cross_inner_selected == 1 && cross_inner_unselected == 0
        && locked_inner_selected == 1 && locked_inner_unselected == 0
        && locked_inner_comma == 1;
    int post_wrappers = named_post_selected == 1 && named_post_unselected == 0
        && anonymous_post_selected == 1 && anonymous_post_unselected == 0
        && choice_post_comma == 1
        && cross_post_selected == 1 && cross_post_unselected == 0
        && locked_post_selected == 1 && locked_post_unselected == 0
        && locked_post_comma == 1;
    return score + captures + inner_wrappers + post_wrappers;
}
