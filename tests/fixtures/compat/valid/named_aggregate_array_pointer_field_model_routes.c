struct Fields {
    int *cursor;
    const int *reader;
};

struct NamedHolder {
    struct Fields nested;
};

struct AnonymousHolder {
    struct {
        int *cursor;
        const int *reader;
    } nested;
};

union Choice {
    struct Fields nested;
    int marker;
};

int *forward_mut(int *value) {
    return value;
}

const int *forward_const(const int *value) {
    return value;
}

int main(void) {
    int named_values[6] = {2, 3, 4, 5, 6, 7};
    int anonymous_values[6] = {12, 13, 14, 15, 16, 17};
    int union_values[6] = {22, 23, 24, 25, 26, 27};
    struct NamedHolder named[2] = {
        {.nested = {named_values, named_values}},
        {.nested = {named_values + 1, named_values + 1}},
    };
    struct AnonymousHolder anonymous[2] = {
        {.nested = {anonymous_values, anonymous_values}},
        {.nested = {anonymous_values + 1, anonymous_values + 1}},
    };
    union Choice choices[2] = {
        {.nested = {union_values, union_values}},
        {.nested = {union_values + 1, union_values + 1}},
    };
    int checks = 0;

    checks += named[1].nested.cursor[1] == 4;
    checks += 1[anonymous[1].nested.cursor] == 14;
    int *slot = &choices[0].nested.cursor[2];
    checks += *slot == 24;
    *slot = 30;
    checks += choices[0].nested.cursor[2] == 30;
    checks += (named[0].nested.cursor[1] += 4) == 7;
    checks += anonymous[0].nested.cursor[2]++ == 14;
    checks += ++anonymous[0].nested.cursor[2] == 16;

    int *replaced = (named[0].nested.cursor = anonymous_values + 1);
    int *compounded = (anonymous[0].nested.cursor += 2);
    int *old = choices[0].nested.cursor++;
    int *incremented = ++choices[0].nested.cursor;
    checks += replaced == anonymous_values + 1;
    checks += compounded == anonymous_values + 2;
    checks += old == union_values;
    checks += incremented == union_values + 2;

    int marker = 0;
    int *mutable_forwarded = forward_mut(
        (marker += 1, 1 ? named[0].nested.cursor : anonymous[1].nested.cursor));
    const int *const_forwarded = forward_const(
        (marker += 1, 0 ? named[1].nested.reader : choices[1].nested.reader));
    checks += mutable_forwarded == anonymous_values + 1;
    checks += const_forwarded == union_values + 1;
    checks += marker == 2;

    checks += sizeof(named[0].nested.cursor[0]) == sizeof(int);
    checks += sizeof(anonymous[0].nested.cursor) == sizeof(int *);
    checks += sizeof(&choices[0].nested.cursor[0]) == sizeof(int *);
    checks += sizeof(forward_mut(choices[0].nested.cursor)) == sizeof(int *);
    checks += sizeof(*forward_const(named[0].nested.reader)) == sizeof(int);
    checks += sizeof(named) / sizeof(named[0]) == 2;

    return checks;
}
