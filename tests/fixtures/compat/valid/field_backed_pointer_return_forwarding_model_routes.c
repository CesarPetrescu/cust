struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

struct IntFieldHolder {
    int primary[4];
    int secondary[4];
};

struct CharFieldHolder {
    char primary[4];
    char secondary[4];
};

struct PointFieldHolder {
    struct Point primary[4];
    struct Point secondary[4];
};

struct NumberFieldHolder {
    union Number primary[4];
    union Number secondary[4];
};

int *forward_field_int(int *value) {
    return value;
}

int *forward_field_int_twice(int *value) {
    return forward_field_int(value);
}

const int *forward_const_field_int(const int *value) {
    return value;
}

const int *forward_const_field_int_twice(const int *value) {
    return forward_const_field_int(value);
}

char *forward_field_char(char *value) {
    return value;
}

char *forward_field_char_twice(char *value) {
    return forward_field_char(value);
}

const char *forward_const_field_char(const char *value) {
    return value;
}

const char *forward_const_field_char_twice(const char *value) {
    return forward_const_field_char(value);
}

struct Point *forward_field_point(struct Point *value) {
    return value;
}

struct Point *forward_field_point_twice(struct Point *value) {
    return forward_field_point(value);
}

const struct Point *forward_const_field_point(const struct Point *value) {
    return value;
}

const struct Point *forward_const_field_point_twice(const struct Point *value) {
    return forward_const_field_point(value);
}

union Number *forward_field_number(union Number *value) {
    return value;
}

union Number *forward_field_number_twice(union Number *value) {
    return forward_field_number(value);
}

const union Number *forward_const_field_number(const union Number *value) {
    return value;
}

const union Number *forward_const_field_number_twice(const union Number *value) {
    return forward_const_field_number(value);
}

int main(void) {
    struct IntFieldHolder ints = {
        .primary = {1, 2, 3, 4},
        .secondary = {5, 6, 7, 8},
    };
    struct IntFieldHolder *ints_view = &ints;
    int choose = 1;
    int marker = 0;
    int *i = forward_field_int_twice(ints_view->primary + 1);
    const int *ci = forward_const_field_int(
        choose ? ints.secondary + 2 : ints_view->primary);
    int *ic =
        (marker += forward_field_int(ints.primary + 3) -
                   forward_field_int_twice(ints_view->primary + 1),
         forward_field_int_twice(ints.primary));
    int *ia = &forward_field_int(ints_view->secondary + 1)[1];
    int total = *i + *ci + *ic + *ia + marker;

    struct CharFieldHolder chars = {
        .primary = {2, 3, 4, 5},
        .secondary = {6, 7, 8, 9},
    };
    struct CharFieldHolder *chars_view = &chars;
    char *c = forward_field_char(chars_view->primary + 2);
    const char *cc = forward_const_field_char_twice(chars.secondary + 1);
    const char *selected_char =
        choose ? forward_const_field_char(chars.primary)
               : forward_const_field_char_twice(chars_view->secondary + 3);
    total += *c + *cc + *selected_char + (c - chars.primary);

    struct PointFieldHolder points = {
        .primary = {{10}, {11}, {12}, {13}},
        .secondary = {{14}, {15}, {16}, {17}},
    };
    struct PointFieldHolder *points_view = &points;
    struct Point *p = forward_field_point_twice(points_view->primary + 3);
    const struct Point *cp = forward_const_field_point(points.secondary);
    struct Point *pa = &forward_field_point(points_view->secondary + 1)[1];
    marker = forward_field_point(points.primary + 3) -
             forward_field_point_twice(points_view->primary + 1);
    p->value = 18;
    total += p->value + cp->value + pa->value + marker;

    struct NumberFieldHolder numbers = {
        .primary = {{20}, {21}, {22}, {23}},
        .secondary = {{24}, {25}, {26}, {27}},
    };
    struct NumberFieldHolder *numbers_view = &numbers;
    union Number *n = forward_field_number(numbers_view->secondary + 1);
    const union Number *cn =
        forward_const_field_number_twice(numbers.primary + 2);
    const union Number *selected_number =
        choose ? forward_const_field_number(numbers_view->primary + 3)
               : forward_const_field_number_twice(numbers.secondary);
    n->value = 28;
    total += n->value + cn->value + selected_number->value +
             (selected_number - numbers.primary);

    return total;
}
