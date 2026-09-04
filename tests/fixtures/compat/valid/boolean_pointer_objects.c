_Bool global_values[4] = {0, 0, 0, 0};
_Bool *global_slot = 0;
_Bool **global_output = &global_slot;
static _Bool *file_slot = 0;
static _Bool **file_output = &file_slot;

static void set_output(_Bool **output, _Bool *value) {
    if (output) {
        *output = value;
    }
}

static void redirect_output(_Bool **output, _Bool **replacement, _Bool *value) {
    output = replacement;
    *output = value;
}

static int update_static(_Bool *value) {
    static _Bool *local_slot = 0;
    static _Bool **local_output = &local_slot;
    set_output(local_output, value + 2);
    **local_output = 9;
    return *local_slot;
}

int main(void) {
    _Bool local_values[4] = {0, 0, 0, 0};
    _Bool *local_slot = 0;
    _Bool *redirect_slot = 0;
    _Bool **local_output = &local_slot;
    _Bool **alias = local_output;
    _Bool **null_output = 0;

    set_output(local_output, local_values + 1);
    **local_output = 7;
    set_output(global_output, global_values + 1);
    **global_output = -3;
    *file_output = global_values + 2;
    **file_output = 11;
    redirect_output(local_output, &redirect_slot, local_values + 3);
    *redirect_slot = 5;

    if (local_values[1] != 1 || global_values[1] != 1 || global_values[2] != 1 ||
        local_values[3] != 1) {
        return 1;
    }
    if (alias != &local_slot || local_output != &local_slot || null_output != 0 ||
        null_output || sizeof(local_output) != sizeof(&local_slot) ||
        sizeof(*local_output) != sizeof(local_slot) ||
        sizeof(**local_output) != sizeof(*local_slot)) {
        return 2;
    }
    return update_static(local_values) == 1 ? 0 : 3;
}
