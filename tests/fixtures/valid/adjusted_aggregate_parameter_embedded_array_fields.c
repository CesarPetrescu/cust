struct Point {
    int value;
};

struct Inner {
    int values[3];
    struct Point points[2];
};

struct Holder {
    struct Inner nested;
};

union Choice {
    struct Inner nested;
    int marker;
};

int score(struct Point point) {
    return point.value;
}

int update_values(struct Holder items[]) {
    int outer = 0;
    int inner = 1;
    int rhs = 0;
    int *slot = &items[outer++].nested.values[inner++];
    *slot += 4;
    int direct = items[0].nested.values[0] = (rhs++, 7);
    int compound = 1[items[0].nested.values] += 3;
    int old = items[0].nested.values[2]++;
    return direct + compound + old + *slot + outer + inner + rhs;
}

int update_points(union Choice items[], struct Point replacement) {
    int outer = 0;
    int inner = 1;
    int rhs = 0;
    struct Point assigned = (inner++)[items[outer++].nested.points] =
        (rhs++, replacement);
    struct Point copied = items[0].nested.points[1];
    struct Point *slot = &0[items[0].nested.points];
    slot->value += 2;
    assigned.value = 11;
    return score(copied) + score(items[0].nested.points[1])
        + slot->value + assigned.value + outer + inner + rhs;
}

int inspect(union Choice items[]) {
    int ok = 0;
    ok += sizeof(items[0].nested.values[0]) == sizeof(int);
    ok += sizeof(items[0].nested.points[0]) == sizeof(struct Point);
    return ok;
}

int main(void) {
    struct Holder holders[1] = {
        {.nested = {{3, 5, 7}, {{3}, {7}}}}
    };
    union Choice choices[1] = {
        {.nested = {{2, 4, 6}, {{3}, {7}}}}
    };
    struct Point replacement = {9};
    return update_values(holders) + update_points(choices, replacement)
        + inspect(choices);
}
