struct Point {
    int value;
};

struct Inner {
    int values[3];
    struct Point points[3];
};

struct Item {
    struct Inner nested;
};

struct Holder {
    struct Item items[3];
};

int scalar_direct_marker;
int aggregate_direct_marker;
int scalar_selection_calls;
int aggregate_selection_calls;
int scalar_reforward_calls;
int aggregate_reforward_calls;
int scalar_return_calls;
int aggregate_return_calls;
int scalar_final_selection_calls;
int aggregate_final_selection_calls;

const int *return_int(struct Item items[]) {
    int *raw = &items[0].nested.values[0];
    raw[0] = 5;
    raw[1] = 7;
    raw[2] = 9;
    return raw + 1;
}

const int *return_other_int(struct Item items[]) {
    int *raw = &items[0].nested.values[0];
    raw[0] = 15;
    raw[1] = 17;
    raw[2] = 19;
    return raw + 1;
}

const struct Point *return_point(struct Item items[]) {
    struct Point *raw = &items[0].nested.points[0];
    raw[0].value = 5;
    raw[1].value = 7;
    raw[2].value = 9;
    return raw + 1;
}

const struct Point *return_other_point(struct Item items[]) {
    struct Point *raw = &items[0].nested.points[0];
    raw[0].value = 15;
    raw[1].value = 17;
    raw[2].value = 19;
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

const int *reselect_int(const int *first, const int *second, int select_first) {
    scalar_final_selection_calls = scalar_final_selection_calls + 1;
    return select_first ? first : second;
}

const int *reselect_int_twice(const int *first, const int *second, int select_first) {
    scalar_final_selection_calls = scalar_final_selection_calls + 1;
    return reselect_int(first, second, select_first);
}

const struct Point *reselect_point(
    const struct Point *first,
    const struct Point *second,
    int select_first
) {
    aggregate_final_selection_calls = aggregate_final_selection_calls + 1;
    return select_first ? first : second;
}

int main(void) {
    struct Holder scalar_holder = {};
    struct Holder point_holder = {};

    const int *int_base = return_int(
        (struct Item[3]){
            {},
            {.nested = {.values = {++scalar_direct_marker}}},
            {},
        }
        + 1
    );
    const int *int_selected = select_int_twice(int_base + 1, int_base, 1);
    const int *int_composed = int_selected - 1;
    const int *int_returned = return_int_boundary_twice(reforward_int(int_composed));
    const int *int_alternate = return_other_int(scalar_holder.items + 1);
    const int *int_reselected = reselect_int_twice(int_alternate, int_returned, 1);

    const struct Point *point_base = return_point(point_holder.items + 1);
    const struct Point *point_selected = select_point(point_base + 1, point_base, 1);
    const struct Point *point_composed = point_selected - 1;
    const struct Point *point_returned = return_point_boundary(
        reforward_point_twice(point_composed)
    );
    const struct Point *point_alternate = return_other_point(
        (struct Item[3]){
            {},
            {.nested = {.values = {++aggregate_direct_marker}}},
            {},
        }
        + 1
    );
    const struct Point *point_reselected = reselect_point(
        point_returned,
        point_alternate,
        1
    );

    return (*int_reselected == 17) + (int_reselected == int_alternate)
        + (int_reselected != int_returned) + (*int_returned == 7)
        + (int_returned == int_base) + (*int_selected == 9)
        + (int_selected == int_base + 1) + (int_returned - int_base == 0)
        + (int_selected - int_base == 1) + (scalar_selection_calls == 2)
        + (scalar_reforward_calls == 1) + (scalar_return_calls == 2)
        + (scalar_final_selection_calls == 2) + (scalar_direct_marker == 1)
        + (scalar_holder.items[1].nested.values[1] == 17)
        + (int_alternate != int_base)
        + (point_reselected->value == 7) + (point_reselected == point_returned)
        + (point_reselected != point_alternate) + (point_alternate->value == 17)
        + (point_returned == point_base) + (point_selected->value == 9)
        + (point_selected == point_base + 1) + (point_returned - point_base == 0)
        + (point_selected - point_base == 1) + (aggregate_selection_calls == 1)
        + (aggregate_reforward_calls == 2) + (aggregate_return_calls == 1)
        + (aggregate_final_selection_calls == 1) + (aggregate_direct_marker == 1)
        + (point_holder.items[1].nested.points[1].value == 7)
        + (point_alternate != point_base);
}
