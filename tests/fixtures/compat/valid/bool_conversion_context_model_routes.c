int marker;
int raw[2] = {3, 7};
int *global_cursor = raw;
_Bool global_value;

struct BoolBox {
    _Bool value;
};

int scalar_identity(int value) {
    marker++;
    return value;
}

int *pointer_identity(int *value) {
    marker++;
    return value;
}

int consume_bool(_Bool value) {
    return value;
}

_Bool pointer_result(int *value) {
    return (marker++, value);
}

int static_result(int *value) {
    static _Bool result;
    result = (marker++, value);
    return result;
}

int main(void) {
    int *null_pointer = 0;
    int scalar_source = 4;
    int *pointer_source = null_pointer;
    int checks = 0;

    _Bool automatic = (marker++, -7);
    checks += automatic == 1;
    automatic = (marker++, 0);
    checks += automatic == 0;

    global_value = 1 ? (marker++, global_cursor) : (marker += 20, null_pointer);
    checks += global_value == 1;
    checks += static_result(null_pointer) == 0;

    _Bool values[2] = {0, 0};
    values[0] = (marker++, global_cursor);
    checks += values[0] == 1;
    values[1] = (marker++, null_pointer);
    checks += values[1] == 0;

    struct BoolBox box = {0};
    box.value = scalar_identity(-4);
    checks += box.value == 1;
    struct BoolBox *view = &box;
    view->value = pointer_identity(null_pointer);
    checks += view->value == 0;

    checks += consume_bool((marker++, global_cursor)) == 1;
    checks += pointer_result(global_cursor) == 1;
    checks += pointer_result(null_pointer) == 0;

    _Bool *slot = values;
    *slot = (marker++, global_cursor);
    checks += *slot == 1;

    _Bool scalar_assignment = (scalar_source = (marker++, -9));
    checks += scalar_assignment == 1;
    _Bool pointer_assignment = (pointer_source = (marker++, global_cursor));
    checks += pointer_assignment == 1;
    _Bool scalar_cast = (_Bool)(marker++, -12);
    checks += scalar_cast == 1;
    _Bool pointer_cast = (int *)(marker++, global_cursor);
    checks += pointer_cast == 1;

    automatic = 0;
    int observed = (automatic += 8);
    checks += observed == 1;
    checks += automatic == 1;
    observed = (automatic -= 1);
    checks += observed == 0;
    checks += automatic == 0;

    values[1] = 0;
    observed = (values[1] += 4);
    checks += observed == 1;
    checks += values[1] == 1;

    box.value = 1;
    observed = (box.value -= 1);
    checks += observed == 0;
    checks += box.value == 0;

    *slot = 0;
    observed = (*slot += 2);
    checks += observed == 1;
    checks += *slot == 1;
    checks += marker == 16;
    return checks;
}