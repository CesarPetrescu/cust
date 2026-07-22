struct Point {
    int value;
};

struct Inner {
    int values[4];
    struct Point points[4];
};

struct Item {
    int marker;
    struct Inner nested;
};

struct Holder {
    struct Item items[3];
};

int direct_int_root;
int direct_point_root;
int int_callee_calls;
int point_callee_calls;
int int_outer_calls;
int point_outer_calls;
int int_selected;
int int_unselected;
int point_comma;
int scalar_caller_calls;
int aggregate_caller_calls;

const int *promote_int(int *value) {
    return value;
}

const struct Point *promote_point(struct Point *value) {
    return value;
}

const int *reforward_int(const int *value) {
    return value;
}

const struct Point *reforward_point(const struct Point *value) {
    return value;
}

const int *caller_reforward_int(const int *value) {
    scalar_caller_calls = scalar_caller_calls + 1;
    return value;
}

const int *caller_reforward_int_twice(const int *value) {
    scalar_caller_calls = scalar_caller_calls + 1;
    return caller_reforward_int(value);
}

const struct Point *caller_reforward_point(const struct Point *value) {
    aggregate_caller_calls = aggregate_caller_calls + 1;
    return value;
}

const struct Point *caller_reforward_point_twice(const struct Point *value) {
    aggregate_caller_calls = aggregate_caller_calls + 1;
    return caller_reforward_point(value);
}

const int *return_int(struct Item items[]) {
    int_callee_calls = int_callee_calls + 1;
    int *raw = &items[0].nested.values[0];
    raw[0] = 5;
    raw[1] = 7;
    raw[2] = 9;
    raw[3] = 11;
    const int *promoted = promote_int(raw) + 1;
    return reforward_int(promoted) + 1;
}

const int *return_int_twice(struct Item items[]) {
    int_callee_calls = int_callee_calls + 1;
    return return_int(items);
}

const struct Point *return_point(struct Item items[]) {
    point_callee_calls = point_callee_calls + 1;
    struct Point *raw = &items[0].nested.points[0];
    raw[0].value = 5;
    raw[1].value = 7;
    raw[2].value = 9;
    raw[3].value = 11;
    const struct Point *promoted = promote_point(raw) + 1;
    return reforward_point(promoted) + 1;
}

const struct Point *return_point_twice(struct Item items[]) {
    point_callee_calls = point_callee_calls + 1;
    return return_point(items);
}

const int *outer_int(const int *base) {
    int_outer_calls = int_outer_calls + 1;
    return 1 + caller_reforward_int(0
            ? (++int_unselected, (const int *)0)
            : (++int_selected, base));
}

const int *outer_int_twice(const int *base) {
    int_outer_calls = int_outer_calls + 1;
    return outer_int(base);
}

const struct Point *outer_point(const struct Point *base) {
    point_outer_calls = point_outer_calls + 1;
    return caller_reforward_point((++point_comma, base)) + 1;
}

const struct Point *outer_point_twice(const struct Point *base) {
    point_outer_calls = point_outer_calls + 1;
    return outer_point(base);
}

int main(void) {
    struct Holder holder = {
        .items = {
            {},
            {.marker = 1},
            {},
        },
    };

    const int *direct_int_base = return_int(
        (struct Item[3]){
            {},
            {.marker = ++direct_int_root},
            {},
        }
        + 1
    );
    const int *direct_int = outer_int(direct_int_base);

    const int *captured_int_base = return_int_twice(holder.items + 1);
    const int *captured_int = outer_int_twice(captured_int_base);

    const struct Point *direct_point_base = return_point(
        &((struct Item[3]){
            {},
            {.marker = ++direct_point_root},
            {},
        })[1]
    );
    const struct Point *direct_point = outer_point_twice(direct_point_base);

    const struct Point *captured_point_base = return_point_twice(holder.items + 1);
    const struct Point *captured_point = outer_point(captured_point_base);

    return (*direct_int == 11) + (direct_int == direct_int_base + 1)
        + (direct_int_root == 1)
        + (*captured_int == 11) + (captured_int == captured_int_base + 1)
        + (holder.items[1].nested.values[2] == 9)
        + (direct_point->value == 11)
        + (direct_point == direct_point_base + 1)
        + (direct_point_root == 1)
        + (captured_point->value == 11)
        + (captured_point == captured_point_base + 1)
        + (holder.items[1].nested.points[2].value == 9)
        + (int_callee_calls == 3) + (point_callee_calls == 3)
        + (int_outer_calls == 3) + (point_outer_calls == 3)
        + (int_selected == 2 && int_unselected == 0)
        + (point_comma == 2)
        + (scalar_caller_calls == 2)
        + (aggregate_caller_calls == 2);
}
