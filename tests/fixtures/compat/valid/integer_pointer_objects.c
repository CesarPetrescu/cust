int global_values[4] = {20, 21, 22, 23};
int *global_slot = 0, *global_second_slot = 0;
int **global_output = &global_slot, **global_second_output = &global_second_slot;
static int *file_slot = 0;
static int **file_output = &file_slot;

static void set_output(int **output, int *value) {
    if (output) {
        *output = value;
    }
}

static void redirect_output(int **output, int **replacement, int *value) {
    output = replacement;
    *output = value;
}

static int update_static(int *value) {
    static int *local_slot = 0;
    static int **local_output = &local_slot;
    set_output(local_output, value + 2);
    return *local_slot;
}

int main(void) {
    int local_values[4] = {10, 11, 12, 13};
    int *local_slot = 0, *local_second_slot = 0, *mixed_slot = 0, *redirect_slot = 0,
        **mixed_output = &mixed_slot;
    int **local_output = &local_slot, **local_second_output = &local_second_slot;
    int **alias = local_output;
    int **null_output = 0;
    int markers = 0;
    int **selected = (markers++, 1 ? local_output : local_second_output);

    set_output(0 ? local_second_output : selected, local_values + 1);
    set_output(global_output, global_values + 1);
    *global_second_output = global_values + 2;
    set_output((markers++, local_second_output), local_values + 2);
    *mixed_output = local_values + 3;
    *file_output = global_values + 3;
    set_output(null_output, local_values);

    local_output = 0;
    if (local_output != 0) {
        return 5;
    }
    local_output = alias;
    local_output = (markers++, &local_slot);
    redirect_output(local_output, &redirect_slot, local_values + 3);

    if (markers != 3 || alias != selected || local_output != &local_slot ||
        *local_slot != 11 || *local_second_slot != 12 || *mixed_slot != 13 ||
        *redirect_slot != 13 || *global_slot != 21 || *global_second_slot != 22 ||
        *file_slot != 23) {
        return 1;
    }
    if (null_output != 0 || null_output || sizeof(local_output) != sizeof(&local_slot) ||
        sizeof(*local_output) != sizeof(local_slot) ||
        sizeof(**local_output) != sizeof(*local_slot)) {
        return 2;
    }
    if (local_output != &local_slot || &local_slot != local_output ||
        local_output == &local_second_slot || &local_second_slot == local_output) {
        return 3;
    }
    return update_static(local_values) == 12 ? 0 : 4;
}
