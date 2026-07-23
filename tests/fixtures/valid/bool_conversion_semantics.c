_Bool global_value = 7;

struct Flags {
    _Bool direct;
    _Bool values[2];
};

_Bool scalar_result(int value) {
    return value;
}

_Bool pointer_result(int *value) {
    return value;
}

int take_bool(_Bool value) {
    return value;
}

int main(void) {
    int raw[1] = {5};
    int *pointer = raw;
    int *null_pointer = 0;
    _Bool values[3] = {-2, 0, pointer};
    struct Flags flags = {3, {0, -4}};
    int checks = 0;

    checks += global_value == 1;
    checks += values[0] == 1;
    checks += values[1] == 0;
    checks += values[2] == 1;
    checks += flags.direct == 1;
    checks += flags.values[0] == 0;
    checks += flags.values[1] == 1;
    checks += take_bool(-9) == 1;
    checks += take_bool(pointer) == 1;
    checks += take_bool(null_pointer) == 0;
    checks += scalar_result(-8) == 1;
    checks += pointer_result(pointer) == 1;
    checks += pointer_result(null_pointer) == 0;
    checks += (_Bool)-7 == 1;
    checks += (_Bool)pointer == 1;
    checks += (_Bool)null_pointer == 0;
    checks += (_Bool){6} == 1;
    checks += (_Bool){null_pointer} == 0;

    values[0] = 0;
    checks += values[0] == 0;
    values[1] = pointer;
    checks += values[1] == 1;
    flags.direct = pointer;
    checks += flags.direct == 1;

    _Bool scalar = 9;
    checks += scalar == 1;
    scalar = null_pointer;
    checks += scalar == 0;
    scalar = pointer;
    checks += scalar == 1;

    _Bool *slot = values;
    *slot = 8;
    checks += *slot == 1;
    *slot = pointer;
    checks += *slot == 1;
    *slot = null_pointer;
    checks += *slot == 0;
    return checks;
}
