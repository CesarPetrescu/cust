struct Point {
    int value;
};

struct Inner {
    int values[3];
    struct Point points[3];
};

struct Item {
    int marker;
    struct Inner nested;
};

struct Holder {
    struct Item items[3];
};

int direct_root_marker;
int scalar_selection_calls;
int aggregate_selection_calls;
int scalar_wrapper_selected;
int scalar_wrapper_unselected;
int aggregate_wrapper_comma;
int scalar_reforward_calls;
int aggregate_reforward_calls;
int scalar_return_calls;
int aggregate_return_calls;

const int *return_int(struct Item items[]) {
    int *raw = &items[0].nested.values[0];
    raw[0] = 5;
    raw[1] = 7;
    raw[2] = 9;
    return raw + 1;
}

const struct Point *return_point(struct Item items[]) {
    struct Point *raw = &items[0].nested.points[0];
    raw[0].value = 5;
    raw[1].value = 7;
    raw[2].value = 9;
    return raw + 1;
}

const int *select_int(const int *first, const int *second, int select_first) {
    scalar_selection_calls = scalar_selection_calls + 1;
    return select_first ? first : second;
}

const int *select_int_twice(const int *first, const int *second, int select_first) {
    scalar_selection_calls = scalar_selection_calls + 1;
    return select_int(first, second, select_first);
}

const struct Point *select_point(
    const struct Point *first,
    const struct Point *second,
    int select_first
) {
    aggregate_selection_calls = aggregate_selection_calls + 1;
    return select_first ? first : second;
}

const struct Point *select_point_twice(
    const struct Point *first,
    const struct Point *second,
    int select_first
) {
    aggregate_selection_calls = aggregate_selection_calls + 1;
    return select_point(first, second, select_first);
}

const int *reforward_int(const int *value) {
    scalar_reforward_calls = scalar_reforward_calls + 1;
    return value;
}

const struct Point *reforward_point(const struct Point *value) {
    aggregate_reforward_calls = aggregate_reforward_calls + 1;
    return value;
}

const struct Point *reforward_point_twice(const struct Point *value) {
    aggregate_reforward_calls = aggregate_reforward_calls + 1;
    return reforward_point(value);
}

const int *return_int_boundary(const int *value) {
    scalar_return_calls = scalar_return_calls + 1;
    return value;
}

const int *return_int_boundary_twice(const int *value) {
    scalar_return_calls = scalar_return_calls + 1;
    return return_int_boundary(value);
}

const struct Point *return_point_boundary(const struct Point *value) {
    aggregate_return_calls = aggregate_return_calls + 1;
    return value;
}

int main(void) {
    struct Holder holder = {
        .items = {
            {},
            {.marker = 1},
            {},
        },
    };

    const int *int_base = return_int(
        (struct Item[3]){
            {},
            {.marker = ++direct_root_marker},
            {},
        }
        + 1
    );
    const int *int_final = int_base + 1;
    const int *int_first = select_int_twice(int_final, int_base, 1);
    const int *int_alternate = int_base;
    const int *int_selected = select_int(int_alternate, int_first, 0);
    const int *int_composed = (1
        ? (++scalar_wrapper_selected, int_selected)
        : (++scalar_wrapper_unselected, int_alternate)) - 1;
    const int *int_reforwarded = reforward_int(int_composed);
    const int *int_returned = return_int_boundary_twice(int_reforwarded);

    const struct Point *point_base = return_point(holder.items + 1);
    const struct Point *point_final = point_base + 1;
    const struct Point *point_first = select_point(point_final, point_base, 0);
    const struct Point *point_alternate = point_final;
    const struct Point *point_selected = select_point_twice(
        point_first,
        point_alternate,
        1
    );
    const struct Point *point_composed = (
        ++aggregate_wrapper_comma,
        point_selected + 1
    );
    const struct Point *point_reforwarded = reforward_point_twice(point_composed);
    const struct Point *point_returned = return_point_boundary(point_reforwarded);

    return (*int_selected == 9) + (int_selected == int_final)
        + (*int_composed == 7) + (int_composed == int_base)
        + (int_reforwarded == int_composed) + (int_returned == int_composed)
        + (*int_returned == 7) + (int_returned - int_base == 0)
        + (int_selected - int_composed == 1)
        + (scalar_wrapper_selected == 1 && scalar_wrapper_unselected == 0)
        + (direct_root_marker == 1) + (scalar_selection_calls == 3)
        + (scalar_reforward_calls == 1) + (scalar_return_calls == 2)
        + (*int_final == 9) + (*int_base == 7)
        + (int_final == int_base + 1)
        + (int_first == int_final) + (int_alternate == int_base)
        + (point_selected->value == 7) + (point_selected == point_base)
        + (point_composed->value == 9) + (point_composed == point_final)
        + (point_reforwarded == point_composed) + (point_returned == point_composed)
        + (point_returned->value == 9) + (point_returned - point_base == 1)
        + (point_composed - point_selected == 1)
        + (aggregate_wrapper_comma == 1)
        + (holder.items[1].nested.points[1].value == 7)
        + (aggregate_selection_calls == 3)
        + (aggregate_reforward_calls == 2) + (aggregate_return_calls == 1)
        + (point_final->value == 9) + (point_base->value == 7)
        + (point_final == point_base + 1)
        + (point_first == point_base) + (point_alternate == point_final);
}
