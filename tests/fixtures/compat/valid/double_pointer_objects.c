double global_values[4] = {0.25, 0.5, 0.75, 1.0};
double *global_slot = 0;
double **global_output = &global_slot;
static double *file_slot = 0;
static double **file_output = &file_slot;

static void set_output(double **output, double *value) {
    if (output) {
        *output = value;
    }
}

static void redirect_output(double **output, double **replacement, double *value) {
    output = replacement;
    *output = value;
}

static int update_static(double *value) {
    static double *local_slot = 0;
    static double **local_output = &local_slot;
    set_output(local_output, value + 2);
    **local_output = 6.25;
    return *local_slot == 6.25;
}

int main(void) {
    double local_values[4] = {1.25, 2.5, 3.75, 5.0};
    double *local_slot = 0;
    double *redirect_slot = 0;
    double **local_output = &local_slot;
    double **alias = local_output;
    double **null_output = 0;

    set_output(local_output, local_values + 1);
    **local_output = 9.5;
    set_output(global_output, global_values + 1);
    **global_output = -3.5;
    *file_output = global_values + 2;
    **file_output = 8.75;
    redirect_output(local_output, &redirect_slot, local_values + 3);
    *redirect_slot = 7.25;

    if (local_values[1] != 9.5 || global_values[1] != -3.5 ||
        global_values[2] != 8.75 || local_values[3] != 7.25) {
        return 1;
    }
    if (alias != &local_slot || local_output != &local_slot || null_output != 0 ||
        null_output || sizeof(local_output) != sizeof(&local_slot) ||
        sizeof(*local_output) != sizeof(local_slot) ||
        sizeof(**local_output) != sizeof(*local_slot)) {
        return 2;
    }
    return update_static(global_values) ? 0 : 3;
}
