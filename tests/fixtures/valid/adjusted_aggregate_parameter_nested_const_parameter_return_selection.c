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
    const int *int_ultimate = select_int(int_alternate, int_first, 0);

    const struct Point *point_base = outer_point_twice(
        return_point_twice(holder.items + 1)
    );
    const struct Point *point_final = final_point(point_base) + 1;
    const struct Point *point_first = select_point(point_final, point_base, 0);
    const struct Point *point_alternate = point_final;
    const struct Point *point_ultimate = select_point_twice(
        point_first,
        point_alternate,
        1
    );

    return (*int_ultimate == 9) + (int_ultimate == int_first)
        + (int_ultimate == int_final) + (int_ultimate - int_base == 1)
        + (int_ultimate > int_base) + (int_first == int_final)
        + (int_alternate == int_base)
        + (point_ultimate->value == 7) + (point_ultimate == point_first)
        + (point_ultimate == point_base) + (point_final - point_ultimate == 1)
        + (point_ultimate < point_final) + (point_first == point_base)
        + (point_alternate == point_final) + (direct_root_marker == 1)
        + (holder.items[1].nested.points[1].value == 7)
        + (scalar_selection_calls == 3) + (aggregate_selection_calls == 3)
        + (scalar_outer_calls == 1) + (aggregate_outer_calls == 2)
        + (scalar_final_calls == 2) + (aggregate_final_calls == 1)
        + (*int_final == 9) + (*int_base == 7)
        + (point_final->value == 9) + (point_base->value == 7)
        + (int_final == int_base + 1) + (point_final == point_base + 1)
        + (int_ultimate - int_first == 0)
        + (point_ultimate - point_first == 0);
}
