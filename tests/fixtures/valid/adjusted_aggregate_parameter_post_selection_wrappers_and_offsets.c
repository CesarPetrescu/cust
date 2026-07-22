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
int scalar_outer_calls;
int aggregate_outer_calls;
int scalar_final_calls;
int aggregate_final_calls;
int scalar_selection_calls;
int aggregate_selection_calls;
int scalar_post_selected;
int scalar_post_unselected;
int aggregate_post_selected;
int aggregate_post_unselected;

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

const struct Point *return_point_twice(struct Item items[]) {
    return return_point(items);
}

const int *outer_int(const int *value) {
    scalar_outer_calls = scalar_outer_calls + 1;
    return value;
}

const struct Point *outer_point(const struct Point *value) {
    aggregate_outer_calls = aggregate_outer_calls + 1;
    return value;
}

const struct Point *outer_point_twice(const struct Point *value) {
    aggregate_outer_calls = aggregate_outer_calls + 1;
    return outer_point(value);
}

const int *final_int(const int *value) {
    scalar_final_calls = scalar_final_calls + 1;
    return value;
}

const int *final_int_twice(const int *value) {
    scalar_final_calls = scalar_final_calls + 1;
    return final_int(value);
}

const struct Point *final_point(const struct Point *value) {
    aggregate_final_calls = aggregate_final_calls + 1;
    return value;
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

int main(void) {
    struct Holder holder = {
        .items = {
            {},
            {.marker = 1},
            {},
        },
    };

    const int *int_base = outer_int(return_int(
        (struct Item[3]){
            {},
            {.marker = ++direct_root_marker},
            {},
        }
        + 1
    ));
    const int *int_final = final_int_twice(int_base) + 1;
    const int *int_first = select_int_twice(int_final, int_base, 1);
    const int *int_alternate = int_base;
    const int *int_selected = select_int(int_alternate, int_first, 0);
    const int *int_wrapped = 1
        ? (++scalar_post_selected, int_selected)
        : (++scalar_post_unselected, int_alternate);
    const int *int_composed = int_wrapped - 1;

    const struct Point *point_base = outer_point_twice(
        return_point_twice(holder.items + 1)
    );
    const struct Point *point_final = final_point(point_base) + 1;
    const struct Point *point_first = select_point(point_final, point_base, 0);
    const struct Point *point_alternate = point_final;
    const struct Point *point_selected = select_point_twice(
        point_first,
        point_alternate,
        1
    );
    const struct Point *point_composed = 0
        ? (++aggregate_post_unselected, point_alternate - 1)
        : (++aggregate_post_selected, point_selected + 1);

    return (*int_selected == 9) + (int_selected == int_final)
        + (*int_composed == 7) + (int_composed == int_base)
        + (int_selected - int_composed == 1) + (int_composed < int_selected)
        + (scalar_post_selected == 1) + (scalar_post_unselected == 0)
        + (point_selected->value == 7) + (point_selected == point_base)
        + (point_composed->value == 9) + (point_composed == point_final)
        + (point_composed - point_selected == 1) + (point_selected < point_composed)
        + (aggregate_post_selected == 1) + (aggregate_post_unselected == 0)
        + (int_first == int_final) + (int_alternate == int_base)
        + (point_first == point_base) + (point_alternate == point_final)
        + (direct_root_marker == 1)
        + (holder.items[1].nested.points[1].value == 7)
        + (scalar_selection_calls == 3) + (aggregate_selection_calls == 3)
        + (scalar_outer_calls == 1) + (aggregate_outer_calls == 2)
        + (scalar_final_calls == 2) + (aggregate_final_calls == 1)
        + (*int_final == 9) + (*int_base == 7)
        + (point_final->value == 9) + (point_base->value == 7)
        + (int_final == int_base + 1) + (point_final == point_base + 1)
        + (int_composed - int_base == 0)
        + (point_final - point_composed == 0)
        + (int_selected - int_first == 0)
        + (point_selected - point_first == 0)
        + (int_composed != int_selected)
        + (point_composed != point_selected);
}
