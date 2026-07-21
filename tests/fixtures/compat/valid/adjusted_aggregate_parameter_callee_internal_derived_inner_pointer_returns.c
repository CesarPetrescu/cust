struct Point {
    int value;
};

struct Inner {
    int values[3];
    struct Point points[3];
};

struct Item {
    int capture;
    struct Inner nested;
};

struct Holder {
    struct Item items[3];
};

int values_before_selected;
int values_before_unselected;
int values_after_comma;
int points_after_selected;
int points_after_unselected;
int points_before_selected;
int points_before_unselected;

const int *promote_int(int *value) {
    return value;
}

const int *promote_int_twice(int *value) {
    return promote_int(value);
}

const struct Point *promote_point(struct Point *value) {
    return value;
}

const int *return_values_before(struct Item items[]) {
    int *raw = &items[0].nested.values[0];
    return (1
            ? (++values_before_selected, promote_int(raw))
            : (++values_before_unselected, promote_int(raw + 2)))
        + 1;
}

const int *return_values_after(struct Item items[]) {
    int *raw = &items[0].nested.values[0];
    return promote_int_twice((++values_after_comma, raw)) + 1;
}

const int *return_values_after_twice(struct Item items[]) {
    return return_values_after(items);
}

const struct Point *return_points_after_offset(struct Item items[]) {
    struct Point *raw = &items[0].nested.points[0];
    return promote_point((0
            ? (++points_after_unselected, raw + 2)
            : (++points_after_selected, raw))
        + 1);
}

const struct Point *return_points_before(struct Item items[]) {
    struct Point *raw = &items[0].nested.points[0];
    return (1
            ? (++points_before_selected, promote_point(raw))
            : (++points_before_unselected, promote_point(raw + 2)))
        + 1;
}

const struct Point *return_points_before_twice(struct Item items[]) {
    return return_points_before(items);
}

int main(void) {
    struct Holder holder = {
        .items = {
            {.nested = {.values = {3}, .points = {{13}}}},
            {.nested = {.values = {5, 7, 9}, .points = {{11}, {17}, {19}}}},
            {},
        },
    };
    int direct_values_marker = 0;
    int direct_points_marker = 0;
    int caller_a_comma = 0;
    int caller_b_selected = 0;
    int caller_b_unselected = 0;
    int caller_c_selected = 0;
    int caller_c_unselected = 0;
    int caller_d_comma = 0;

    const int *a_base = return_values_before(
        (struct Item[3]){
            {},
            {.capture = ++direct_values_marker, .nested = {.values = {5, 7, 9}}},
            {},
        }
        + 1
    );
    const int *a = (++caller_a_comma, a_base) + 1;

    const int *b_base = return_values_after_twice(holder.items + 1);
    const int *b = 1 + (0
            ? (++caller_b_unselected, (const int *)0)
            : (++caller_b_selected, b_base));

    const struct Point *c_base = return_points_after_offset(
        &((struct Item[3]){
            {},
            {.capture = ++direct_points_marker, .nested = {.points = {{11}, {17}, {19}}}},
            {},
        })[1]
    );
    const struct Point *c = &(1
            ? (++caller_c_selected, c_base)
            : (++caller_c_unselected, (const struct Point *)0))[1];

    const struct Point *d_base = return_points_before_twice(holder.items + 1);
    const struct Point *d = &((++caller_d_comma, d_base)[1]);

    return (a == a_base + 1) + (*a == 9)
        + (values_before_selected == 1 && values_before_unselected == 0)
        + (caller_a_comma == 1) + (direct_values_marker == 1)
        + (b == b_base + 1) + (*b == 9) + (values_after_comma == 1)
        + (caller_b_selected == 1 && caller_b_unselected == 0)
        + (holder.items[1].nested.values[1] == 7)
        + (c == c_base + 1) + (c->value == 19)
        + (points_after_selected == 1 && points_after_unselected == 0)
        + (caller_c_selected == 1 && caller_c_unselected == 0)
        + (direct_points_marker == 1)
        + (d == d_base + 1) + (d->value == 19)
        + (points_before_selected == 1 && points_before_unselected == 0)
        + (caller_d_comma == 1) + (holder.items[1].nested.points[1].value == 17);
}
