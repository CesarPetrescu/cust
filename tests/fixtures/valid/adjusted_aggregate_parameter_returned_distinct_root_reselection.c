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
int scalar_carry_calls;
int aggregate_carry_calls;
int scalar_return_calls;
int aggregate_return_calls;
int scalar_carry_selected;
int scalar_carry_unselected;
int aggregate_carry_comma;
int scalar_final_selection_calls;
int aggregate_final_selection_calls;
int scalar_fresh_carry_calls;
int aggregate_fresh_carry_calls;
int scalar_fresh_return_calls;
int aggregate_fresh_return_calls;
int scalar_fresh_selected;
int scalar_fresh_unselected;
int aggregate_fresh_comma;
int scalar_fresh_reselection_calls;
int aggregate_fresh_reselection_calls;

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

const int *carry_int(const int *value) {
    scalar_carry_calls = scalar_carry_calls + 1;
    return value;
}

const int *carry_int_twice(const int *value) {
    scalar_carry_calls = scalar_carry_calls + 1;
    return carry_int(value);
}

const struct Point *carry_point(const struct Point *value) {
    aggregate_carry_calls = aggregate_carry_calls + 1;
    return value;
}

const int *return_selected_int(const int *value) {
    scalar_return_calls = scalar_return_calls + 1;
    return value;
}

const struct Point *return_selected_point(const struct Point *value) {
    aggregate_return_calls = aggregate_return_calls + 1;
    return value;
}

const struct Point *return_selected_point_twice(const struct Point *value) {
    aggregate_return_calls = aggregate_return_calls + 1;
    return return_selected_point(value);
}

const int *reselect_carried_int(
    const int *first,
    const int *second,
    int select_first
) {
    scalar_final_selection_calls = scalar_final_selection_calls + 1;
    return select_first ? first : second;
}

const int *reselect_carried_int_twice(
    const int *first,
    const int *second,
    int select_first
) {
    scalar_final_selection_calls = scalar_final_selection_calls + 1;
    return reselect_carried_int(first, second, select_first);
}

const struct Point *reselect_carried_point(
    const struct Point *first,
    const struct Point *second,
    int select_first
) {
    aggregate_final_selection_calls = aggregate_final_selection_calls + 1;
    return select_first ? first : second;
}

const int *carry_final_int(const int *value) {
    scalar_fresh_carry_calls = scalar_fresh_carry_calls + 1;
    return value;
}

const int *carry_final_int_twice(const int *value) {
    scalar_fresh_carry_calls = scalar_fresh_carry_calls + 1;
    return carry_final_int(value);
}

const int *return_final_int(const int *value) {
    scalar_fresh_return_calls = scalar_fresh_return_calls + 1;
    return value;
}

const struct Point *carry_final_point(const struct Point *value) {
    aggregate_fresh_carry_calls = aggregate_fresh_carry_calls + 1;
    return value;
}

const struct Point *return_final_point(const struct Point *value) {
    aggregate_fresh_return_calls = aggregate_fresh_return_calls + 1;
    return value;
}

const struct Point *return_final_point_twice(const struct Point *value) {
    aggregate_fresh_return_calls = aggregate_fresh_return_calls + 1;
    return return_final_point(value);
}

const int *reselect_fresh_int(
    const int *first,
    const int *second,
    int select_first
) {
    scalar_fresh_reselection_calls = scalar_fresh_reselection_calls + 1;
    return select_first ? first : second;
}

const struct Point *reselect_fresh_point(
    const struct Point *first,
    const struct Point *second,
    int select_first
) {
    aggregate_fresh_reselection_calls = aggregate_fresh_reselection_calls + 1;
    return select_first ? first : second;
}

const struct Point *reselect_fresh_point_twice(
    const struct Point *first,
    const struct Point *second,
    int select_first
) {
    aggregate_fresh_reselection_calls = aggregate_fresh_reselection_calls + 1;
    return reselect_fresh_point(first, second, select_first);
}

int main(void) {
    struct Holder scalar_holder = {};
    struct Holder point_holder = {};

    const int *int_primary = return_int(
        (struct Item[3]){
            {},
            {.nested = {.values = {++scalar_direct_marker}}},
            {},
        }
        + 1
    );
    const int *int_alternate = return_other_int(scalar_holder.items + 1);
    const int *int_reselected = select_int_twice(int_alternate, int_primary, 1);
    const int *int_primary_slot = int_primary;
    const int *int_alternate_slot = int_alternate;
    const int *int_carried = carry_int_twice(
        (1
             ? (++scalar_carry_selected, int_reselected)
             : (++scalar_carry_unselected, int_primary))
            + 1
    );
    const int *int_final = return_selected_int(int_carried);
    const int *int_final_reselected = reselect_carried_int_twice(
        int_final,
        int_primary_slot,
        0
    );

    const struct Point *point_primary = return_point(point_holder.items + 1);
    const struct Point *point_alternate = return_other_point(
        (struct Item[3]){
            {},
            {.nested = {.values = {++aggregate_direct_marker}}},
            {},
        }
        + 1
    );
    const struct Point *point_reselected = select_point(
        point_primary,
        point_alternate,
        1
    );
    const struct Point *point_primary_slot = point_primary;
    const struct Point *point_alternate_slot = point_alternate;
    const struct Point *point_carried = carry_point(point_reselected);
    const struct Point *point_composed = &(
        (++aggregate_carry_comma, point_carried)
    )[-1];
    const struct Point *point_final = return_selected_point_twice(point_composed);
    const struct Point *point_final_reselected = reselect_carried_point(
        point_alternate_slot,
        point_final,
        0
    );
    const int *int_fresh_carried = carry_final_int_twice(
        (1
             ? (++scalar_fresh_selected, int_final_reselected)
             : (++scalar_fresh_unselected, int_final))
            + 1
    );
    const int *int_fresh_returned = return_final_int(int_fresh_carried);
    const struct Point *point_fresh_carried = carry_final_point(
        point_final_reselected
    );
    const struct Point *point_fresh_composed = &(
        (++aggregate_fresh_comma, point_fresh_carried)
    )[1];
    const struct Point *point_fresh_returned = return_final_point_twice(
        point_fresh_composed
    );
    const int *int_fresh_slot = int_fresh_returned;
    const int *int_rejected_slot = int_final;
    const int *int_fresh_reselected = reselect_fresh_int(
        int_fresh_returned,
        int_final,
        0
    );
    const struct Point *point_fresh_slot = point_fresh_returned;
    const struct Point *point_rejected_slot = point_alternate_slot;
    const struct Point *point_fresh_reselected = reselect_fresh_point_twice(
        point_alternate_slot,
        point_fresh_returned,
        0
    );

    return (int_reselected == int_alternate) + (int_final == int_alternate + 1)
        + (*int_final == 19) + (int_carried == int_final)
        + (int_primary_slot == int_primary) + (int_alternate_slot == int_alternate)
        + (scalar_carry_selected == 1 && scalar_carry_unselected == 0)
        + (scalar_carry_calls == 2) + (scalar_return_calls == 1)
        + (scalar_selection_calls == 2) + (scalar_direct_marker == 1)
        + (int_primary != int_alternate) + (*int_primary == 7)
        + (*int_alternate == 17) + (scalar_holder.items[1].nested.values[2] == 19)
        + (point_reselected == point_primary) + (point_final == point_primary - 1)
        + (point_final->value == 5) + (point_carried == point_reselected)
        + (point_primary_slot == point_primary)
        + (point_alternate_slot == point_alternate)
        + (aggregate_carry_comma == 1) + (aggregate_carry_calls == 1)
        + (aggregate_return_calls == 2) + (aggregate_selection_calls == 1)
        + (aggregate_direct_marker == 1) + (point_primary != point_alternate)
        + (point_primary->value == 7) + (point_alternate->value == 17)
        + (point_holder.items[1].nested.points[0].value == 5)
        + (point_holder.items[1].nested.points[2].value == 9)
        + (int_final_reselected == int_primary_slot) + (*int_final_reselected == 7)
        + (int_final_reselected != int_final) + (scalar_final_selection_calls == 2)
        + (point_final_reselected == point_final) + (point_final_reselected->value == 5)
        + (point_final_reselected != point_alternate_slot)
        + (aggregate_final_selection_calls == 1)
        + (int_fresh_returned == int_primary_slot + 1) + (*int_fresh_returned == 9)
        + (int_final_reselected == int_primary_slot)
        + (scalar_fresh_selected == 1 && scalar_fresh_unselected == 0)
        + (scalar_fresh_carry_calls == 2) + (scalar_fresh_return_calls == 1)
        + (point_fresh_returned == point_final_reselected + 1)
        + (point_fresh_returned->value == 7)
        + (point_final_reselected == point_final) + (aggregate_fresh_comma == 1)
        + (aggregate_fresh_carry_calls == 1) + (aggregate_fresh_return_calls == 2)
        + (int_fresh_reselected == int_final) + (*int_fresh_reselected == 19)
        + (int_fresh_reselected != int_fresh_returned)
        + (int_fresh_slot == int_fresh_returned) + (int_rejected_slot == int_final)
        + (scalar_fresh_reselection_calls == 1)
        + (point_fresh_reselected == point_fresh_returned)
        + (point_fresh_reselected->value == 7)
        + (point_fresh_reselected != point_alternate_slot)
        + (point_fresh_slot == point_fresh_returned)
        + (point_rejected_slot == point_alternate_slot)
        + (aggregate_fresh_reselection_calls == 2);
}
