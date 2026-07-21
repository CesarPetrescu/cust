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

struct Holder {
    struct Item items[3];
};

int a_first_selected;
int a_first_unselected;
int a_second_selected;
int a_second_unselected;
int b_first_comma;
int b_second_selected;
int b_second_unselected;
int c_first_selected;
int c_first_unselected;
int c_second_comma;
int d_first_comma;
int d_second_comma;

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

const int *reforward_int(const int *value) {
    return value;
}

const int *reforward_int_twice(const int *value) {
    return reforward_int(value);
}

const struct Point *reforward_point(const struct Point *value) {
    return value;
}

const struct Point *reforward_point_twice(const struct Point *value) {
    return reforward_point(value);
}

const int *return_a(struct Item items[]) {
    int *raw = &items[0].nested.values[0];
    raw[0] = 5;
    raw[1] = 7;
    raw[2] = 9;
    raw[3] = 11;
    const int *promoted = (1
            ? (++a_first_selected, promote_int(raw))
            : (++a_first_unselected, promote_int(raw + 2)))
        + 1;
    return (0
            ? (++a_second_unselected, reforward_int(promoted + 1))
            : (++a_second_selected, reforward_int_twice(promoted)))
        + 1;
}

const int *return_b(struct Item items[]) {
    int *raw = &items[0].nested.values[0];
    raw[0] = 5;
    raw[1] = 7;
    raw[2] = 9;
    raw[3] = 11;
    const int *promoted = promote_int_twice((++b_first_comma, raw)) + 1;
    return reforward_int(1
            ? (++b_second_selected, promoted)
            : (++b_second_unselected, promoted + 1))
        + 1;
}

const int *return_b_twice(struct Item items[]) {
    return return_b(items);
}

const struct Point *return_c(struct Item items[]) {
    struct Point *raw = &items[0].nested.points[0];
    raw[0].value = 5;
    raw[1].value = 7;
    raw[2].value = 9;
    raw[3].value = 11;
    const struct Point *promoted = promote_point((0
            ? (++c_first_unselected, raw + 2)
            : (++c_first_selected, raw))
        + 1);
    return reforward_point_twice((++c_second_comma, promoted) + 1);
}

const struct Point *return_d(struct Item items[]) {
    struct Point *raw = &items[0].nested.points[0];
    raw[0].value = 5;
    raw[1].value = 7;
    raw[2].value = 9;
    raw[3].value = 11;
    const struct Point *promoted = promote_point_twice((++d_first_comma, raw) + 1);
    return reforward_point((++d_second_comma, promoted + 1));
}

const struct Point *return_d_twice(struct Item items[]) {
    return return_d(items);
}

int main(void) {
    struct Holder holder = {
        .items = {
            {},
            {.nested = {.values = {1}, .points = {{1}}}},
            {},
        },
    };
    int direct_a_marker = 0;
    int direct_c_marker = 0;
    int caller_a_comma = 0;
    int caller_b_selected = 0;
    int caller_b_unselected = 0;
    int caller_c_selected = 0;
    int caller_c_unselected = 0;
    int caller_d_comma = 0;

    const int *a_base = return_a(
        (struct Item[3]){
            {},
            {.capture = ++direct_a_marker},
            {},
        }
        + 1
    );
    const int *a = (++caller_a_comma, a_base) + 1;

    const int *b_base = return_b_twice(holder.items + 1);
    const int *b = 1 + (0
            ? (++caller_b_unselected, (const int *)0)
            : (++caller_b_selected, b_base));

    const struct Point *c_base = return_c(
        &((struct Item[3]){
            {},
            {.capture = ++direct_c_marker},
            {},
        })[1]
    );
    const struct Point *c = &(1
            ? (++caller_c_selected, c_base)
            : (++caller_c_unselected, (const struct Point *)0))[1];

    const struct Point *d_base = return_d_twice(holder.items + 1);
    const struct Point *d = &((++caller_d_comma, d_base)[1]);

    return (a == a_base + 1) + (*a == 11)
        + (a_first_selected == 1 && a_first_unselected == 0)
        + (a_second_selected == 1 && a_second_unselected == 0)
        + (caller_a_comma == 1) + (direct_a_marker == 1)
        + (b == b_base + 1) + (*b == 11) + (b_first_comma == 1)
        + (b_second_selected == 1 && b_second_unselected == 0)
        + (caller_b_selected == 1 && caller_b_unselected == 0)
        + (holder.items[1].nested.values[2] == 9)
        + (c == c_base + 1) + (c->value == 11)
        + (c_first_selected == 1 && c_first_unselected == 0)
        + (c_second_comma == 1)
        + (caller_c_selected == 1 && caller_c_unselected == 0)
        + (direct_c_marker == 1)
        + (d == d_base + 1) + (d->value == 11)
        + (d_first_comma == 1) + (d_second_comma == 1)
        + (caller_d_comma == 1) + (holder.items[1].nested.points[2].value == 9);
}
