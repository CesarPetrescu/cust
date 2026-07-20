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
    struct Point *point = &items[1].nested.points[1];
    int score = mutate_slots(scalar, point, scalar, point);
    score += scalar == &items[0].nested.values[1];
    score += point == &items[1].nested.points[1];
    score += items[-1].nested.values[0] == 9;
    score += *scalar == 7;
    score += point->value == 10;
    return score;
}

int read_const(const struct Item items[]) {
    return (items[-1].nested.values[0] == 9)
        + (items[0].nested.values[0] == 1)
        + (items[1].nested.points[1].value == 7);
}

int main(void) {
    int named_marker = 0;
    int anonymous_marker = 0;
    int choice_marker = 0;
    int const_marker = 0;

    struct NamedHolder *named = &(struct NamedHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {++named_marker, 5}}},
                {.nested = {.points = {[1] = {7}}}},
            },
        },
    };
    struct AnonymousHolder *anonymous = &(struct AnonymousHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {++anonymous_marker, 5}}},
                {.nested = {.points = {[1] = {7}}}},
            },
        },
    };
    union Choice *choice = &(union Choice){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {++choice_marker, 5}}},
                {.nested = {.points = {[1] = {7}}}},
            },
        },
    };
    const struct NamedHolder *locked = &(const struct NamedHolder){
        .nested = {
            .primary = {
                {.nested = {.values = {9}}},
                {.nested = {.values = {++const_marker}}},
                {.nested = {.points = {[1] = {7}}}},
            },
        },
    };

    int score = probe(named->nested.primary + 1);
    score += probe(1 + anonymous->nested.primary);
    score += probe(&choice->nested.primary[1]);
    score += read_const(&locked->nested.primary[1]);
    score += named_marker == 1;
    score += anonymous_marker == 1;
    score += choice_marker == 1;
    score += const_marker == 1;
    return score;
}
