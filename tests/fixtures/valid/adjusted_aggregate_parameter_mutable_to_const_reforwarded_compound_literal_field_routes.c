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

const struct Item *promote_items(struct Item *items) {
    return items;
}

const struct Item *promote_items_twice(struct Item *items) {
    return promote_items(forward_items(items));
}

const struct Item *forward_const_items(const struct Item *items) {
    return items;
}

int read_promoted(const struct Item items[]) {
    const struct Item *original = items;
    const int *scalar = &items[0].nested.values[1];
    const struct Point *point = &items[0].nested.points[1];
    int score = (items[-1].nested.values[0] == 9)
        + (*scalar == 11)
        + (point->value == 13)
        + (scalar == &items[0].nested.values[1])
        + (point == &items[0].nested.points[1])
        + (original == items);
    const struct Item *fallback = items - 1;
    items = fallback;
    scalar = &fallback[0].nested.values[0];
    point = &fallback[0].nested.points[0];
    return score + (items == fallback)
        + (scalar == &fallback[0].nested.values[0])
        + (point == &fallback[0].nested.points[0]);
}

int main(void) {
    int named_capture = 0;
    int anonymous_capture = 0;
    int choice_capture = 0;

    struct NamedHolder *named = &(struct NamedHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {++named_capture, 11}, .points = {[1] = {13}}}},
                {},
            },
        },
    };
    struct AnonymousHolder *anonymous = &(struct AnonymousHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {++anonymous_capture, 11}, .points = {[1] = {13}}}},
                {},
            },
        },
    };
    union Choice *choice = &(union Choice){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {++choice_capture, 11}, .points = {[1] = {13}}}},
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

    int score = read_promoted(
        forward_const_items(
            1
                ? (named_post_selected++,
                   promote_items(
                       1
                           ? (named_inner_selected++, named->nested.primary)
                           : (named_inner_unselected++, named->nested.secondary)))
                : (named_post_unselected++, promote_items(named->nested.secondary))
        )
        + 1
    );
    score += read_promoted(
        promote_items_twice(
            1
            + (0
                   ? (anonymous_post_unselected++,
                      forward_items_twice(anonymous->nested.secondary))
                   : (anonymous_post_selected++,
                      forward_items_twice(
                          0
                              ? (anonymous_inner_unselected++, anonymous->nested.secondary)
                              : (anonymous_inner_selected++, anonymous->nested.primary)))
        ))
    );
    score += read_promoted(
        promote_items(
            &(choice_post_comma++,
              forward_items((choice_inner_comma++, choice->nested.primary)))[1]
        )
    );

    int captures = named_capture == 1 && anonymous_capture == 1 && choice_capture == 1;
    int wrappers = named_inner_selected == 1 && named_inner_unselected == 0
        && named_post_selected == 1 && named_post_unselected == 0
        && anonymous_inner_selected == 1 && anonymous_inner_unselected == 0
        && anonymous_post_selected == 1 && anonymous_post_unselected == 0
        && choice_inner_comma == 1 && choice_post_comma == 1;
    return score + captures + wrappers;
}
