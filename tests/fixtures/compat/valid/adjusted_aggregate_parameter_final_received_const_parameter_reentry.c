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
int parameter_reentry_calls;

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

int reenter_int(const int *final, const int *base) {
    parameter_reentry_calls = parameter_reentry_calls + 1;
    const int *final_copy = final;
    const int *base_copy = base;
    final = base_copy;
    base = final_copy;
    return (*final_copy == 9) + (*base_copy == 7)
        + (final_copy == base_copy + 1) + (final_copy - base_copy == 1)
        + (final_copy > base_copy) + (base_copy < final_copy)
        + (final == base_copy) + (base == final_copy);
}

int reenter_point(const struct Point *final, const struct Point *base) {
    parameter_reentry_calls = parameter_reentry_calls + 1;
    const struct Point *final_copy = final;
    const struct Point *base_copy = base;
    final = base_copy;
    base = final_copy;
    return (final_copy->value == 9) + (base_copy->value == 7)
        + (final_copy == base_copy + 1) + (final_copy - base_copy == 1)
        + (final_copy > base_copy) + (base_copy < final_copy)
        + (final == base_copy) + (base == final_copy);
}

int reenter_point_twice(const struct Point *final, const struct Point *base) {
    parameter_reentry_calls = parameter_reentry_calls + 1;
    return reenter_point(final, base);
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

    const struct Point *point_base = outer_point_twice(
        return_point_twice(holder.items + 1)
    );
    const struct Point *point_final = final_point(point_base) + 1;

    int score = reenter_int(int_final, int_base)
        + reenter_point_twice(point_final, point_base);

    return score
        + (*int_final == 9) + (*int_base == 7)
        + (int_final == int_base + 1) + (int_final - int_base == 1)
        + (int_final > int_base) + (direct_root_marker == 1)
        + (point_final->value == 9) + (point_base->value == 7)
        + (point_final == point_base + 1) + (point_final - point_base == 1)
        + (point_final > point_base) + (holder.items[1].nested.points[1].value == 7)
        + (parameter_reentry_calls == 3)
        + (scalar_outer_calls == 1) + (aggregate_outer_calls == 2)
        + (scalar_final_calls == 2) + (aggregate_final_calls == 1);
}
