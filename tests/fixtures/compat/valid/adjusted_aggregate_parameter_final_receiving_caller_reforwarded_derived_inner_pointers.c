struct Point {
    int value;
};

struct Inner {
    int values[5];
    struct Point points[5];
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
int int_final_calls;
int point_final_calls;
int int_selected;
int int_unselected;
int point_comma;
int final_int_selected;
int final_int_unselected;
int final_point_comma;
int scalar_caller_calls;
int aggregate_caller_calls;
int scalar_final_calls;
int aggregate_final_calls;

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

const int *final_reforward_int(const int *value) {
    scalar_final_calls = scalar_final_calls + 1;
    return value;
}

const int *final_reforward_int_twice(const int *value) {
    scalar_final_calls = scalar_final_calls + 1;
    return final_reforward_int(value);
}

const struct Point *final_reforward_point(const struct Point *value) {
    aggregate_final_calls = aggregate_final_calls + 1;
    return value;
}

const struct Point *final_reforward_point_twice(const struct Point *value) {
    aggregate_final_calls = aggregate_final_calls + 1;
    return final_reforward_point(value);
}

const int *return_int(struct Item items[]) {
    int_callee_calls = int_callee_calls + 1;
    int *raw = &items[0].nested.values[0];
    raw[0] = 5;
    raw[1] = 7;
    raw[2] = 9;
    raw[3] = 11;
    raw[4] = 13;
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
    raw[4].value = 13;
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

const int *final_int(const int *base) {
    int_final_calls = int_final_calls + 1;
    return final_reforward_int(0
            ? (++final_int_unselected, (const int *)0)
            : (++final_int_selected, base)) + 1;
}

const int *final_int_twice(const int *base) {
    int_final_calls = int_final_calls + 1;
    return final_reforward_int_twice(0
            ? (++final_int_unselected, (const int *)0)
            : (++final_int_selected, base)) + 1;
}

const struct Point *final_point(const struct Point *base) {
    point_final_calls = point_final_calls + 1;
    return 1 + final_reforward_point((++final_point_comma, base));
}

const struct Point *final_point_twice(const struct Point *base) {
    point_final_calls = point_final_calls + 1;
    return 1 + final_reforward_point_twice((++final_point_comma, base));
}

int main(void) {
    struct Holder holder = {
        .items = {
            {},
            {.marker = 1},
            {},
        },
    };

    const int *direct_int_callee = return_int(
        (struct Item[3]){
            {},
            {.marker = ++direct_int_root},
            {},
        }
        + 1
    );
    const int *direct_int_outer = outer_int(direct_int_callee);
    const int *direct_int = final_int(direct_int_outer);

    const int *captured_int_callee = return_int_twice(holder.items + 1);
    const int *captured_int_outer = outer_int_twice(captured_int_callee);
    const int *captured_int = final_int_twice(captured_int_outer);

    const struct Point *direct_point_callee = return_point(
        &((struct Item[3]){
            {},
            {.marker = ++direct_point_root},
            {},
        })[1]
    );
    const struct Point *direct_point_outer = outer_point_twice(direct_point_callee);
    const struct Point *direct_point = final_point_twice(direct_point_outer);

    const struct Point *captured_point_callee = return_point_twice(holder.items + 1);
    const struct Point *captured_point_outer = outer_point(captured_point_callee);
    const struct Point *captured_point = final_point(captured_point_outer);

    return (*direct_int == 13) + (direct_int == direct_int_outer + 1)
        + (direct_int_root == 1)
        + (*captured_int == 13) + (captured_int == captured_int_outer + 1)
        + (holder.items[1].nested.values[2] == 9)
        + (direct_point->value == 13)
        + (direct_point == direct_point_outer + 1)
        + (direct_point_root == 1)
        + (captured_point->value == 13)
        + (captured_point == captured_point_outer + 1)
        + (holder.items[1].nested.points[2].value == 9)
        + (direct_int_outer == direct_int_callee + 1)
        + (captured_int_outer == captured_int_callee + 1)
        + (direct_point_outer == direct_point_callee + 1)
        + (captured_point_outer == captured_point_callee + 1)
        + (int_callee_calls == 3) + (point_callee_calls == 3)
        + (int_outer_calls == 3) + (point_outer_calls == 3)
        + (int_final_calls == 2) + (point_final_calls == 2)
        + (int_selected == 2 && int_unselected == 0)
        + (point_comma == 2)
        + (scalar_caller_calls == 2)
        + (aggregate_caller_calls == 2)
        + (final_int_selected == 2 && final_int_unselected == 0)
        + (final_point_comma == 2)
        + (scalar_final_calls == 3)
        + (aggregate_final_calls == 3);
}
