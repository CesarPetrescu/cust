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

    int named_selected = 0;
    int named_unselected = 0;
    int anonymous_selected = 0;
    int anonymous_unselected = 0;
    int choice_comma = 0;
    int cross_selected = 0;
    int cross_unselected = 0;
    int locked_selected = 0;
    int locked_unselected = 0;
    int locked_comma = 0;

    int score = probe(
        (1
             ? (named_selected++, named->nested.primary)
             : (named_unselected++, anonymous->nested.primary))
        + 1
    );
    score += probe(
        1
        + (0
               ? (anonymous_unselected++, named->nested.primary)
               : (anonymous_selected++, anonymous->nested.primary))
    );
    score += probe(&(choice_comma++, choice->nested.primary)[1]);
    score += probe(
        (1
             ? (cross_selected++, named->nested.secondary)
             : (cross_unselected++, anonymous->nested.secondary))
        + 1
    );
    score += read_const(
        (1
             ? (locked_selected++, locked->nested.primary)
             : (locked_unselected++, locked->nested.secondary))
        + 1
    );
    score += read_const(&(locked_comma++, locked->nested.secondary)[1]);

    int captures = named_capture == 1 && anonymous_capture == 1
        && choice_capture == 1 && locked_capture == 1;
    int wrappers = named_selected == 1 && named_unselected == 0
        && anonymous_selected == 1 && anonymous_unselected == 0
        && choice_comma == 1
        && cross_selected == 1 && cross_unselected == 0
        && locked_selected == 1 && locked_unselected == 0
        && locked_comma == 1;
    return score + captures + wrappers;
}
