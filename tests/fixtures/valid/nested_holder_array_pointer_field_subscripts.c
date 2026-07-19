struct Point {
    int value;
    int extra;
};

struct NamedInner {
    int *values;
    struct Point *points;
};

struct NamedHolder {
    struct NamedInner nested;
};

struct AnonymousHolder {
    struct {
        int *values;
        struct Point *points;
    } nested;
};

union Choice {
    struct NamedInner nested;
    int marker;
};

int consume_point(struct Point point) {
    point.value += 20;
    return point.value + point.extra;
}

struct Point read_choice(union Choice choices[]) {
    return choices[0].nested.points[0];
}

int main(void) {
    int values[4] = {2, 4, 6, 8};
    struct Point points[3] = {{3, 30}, {5, 50}, {7, 70}};
    struct Point replacement = {11, 110};
    struct NamedHolder named[1] = {{.nested = {values, points}}};
    struct AnonymousHolder anonymous[1] = {{.nested = {values, points}}};
    union Choice choices[1] = {{.nested = {values, points}}};
    int total = 0;

    total += named[0].nested.values[1] == 4;
    total += 2[anonymous[0].nested.values] == 6;

    named[0].nested.values[0] = 9;
    total += values[0] == 9;

    total += (anonymous[0].nested.values[1] += 3) == 7;
    total += values[1] == 7;

    int old_value = choices[0].nested.values[2]++;
    total += old_value == 6;
    total += values[2] == 7;

    int new_value = ++choices[0].nested.values[2];
    total += new_value == 8;
    total += values[2] == 8;

    int *slot = &named[0].nested.values[3];
    *slot += 2;
    total += *slot == 10;
    total += values[3] == 10;

    total += consume_point(named[0].nested.points[1]) == 75;
    total += points[1].value == 5;

    struct Point direct = anonymous[0].nested.points[2];
    direct.value = 13;
    total += direct.value == 13;
    total += points[2].value == 7;

    struct Point returned = read_choice(choices);
    returned.extra = 31;
    total += returned.extra == 31;
    total += points[0].extra == 30;

    struct Point conditional = 1 ? named[0].nested.points[0] : replacement;
    conditional.value = 17;
    total += conditional.value == 17;
    total += points[0].value == 3;

    int marker = 0;
    struct Point comma_copy = (marker += 1, anonymous[0].nested.points[1]);
    total += marker == 1;
    total += comma_copy.extra == 50;

    int outer = 0;
    int inner = 0;
    (void)anonymous[outer++].nested.points[inner++];
    total += outer == 1;
    total += inner == 1;

    outer = 0;
    inner = 0;
    int rhs = 0;
    int assigned_extra =
        (choices[outer++].nested.points[inner++] = (rhs++, replacement)).extra;
    total += outer == 1;
    total += inner == 1;
    total += rhs == 1;
    total += assigned_extra == 110;
    total += points[0].value == 11;
    total += points[0].extra == 110;

    total += sizeof(named[0].nested.values[0]) == sizeof(int);
    total += sizeof(anonymous[0].nested.points[0]) == sizeof(struct Point);
    total += sizeof(choices[0].nested.points[0].extra) == sizeof(int);
    return total;
}
