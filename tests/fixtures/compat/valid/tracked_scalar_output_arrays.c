/* Portable pointer identity and same-type size relationships only. */
static int int_first_value = 3;
static int int_second_value = 5;
static int *int_first = &int_first_value;
static int *int_second = &int_second_value;
static int **int_global_outputs[2] = { &int_first, &int_second };

static void set_int(int **output, int *value) { *output = value; }
static int check_int(void) {
    int first_value = 7;
    int second_value = 9;
    int *first = &first_value;
    int *second = &second_value;
    int **outputs[3] = { &first, [2] = &second };
    static int **persistent[2] = { &int_first, &int_second };
    if (sizeof(outputs) != 3 * sizeof(outputs[0])) return 0;
    if (outputs[0] != &first || outputs[1] != 0 || outputs[2] != &second) return 0;
    outputs[1] = outputs[0];
    set_int(outputs[2], &first_value);
    persistent[0] = int_global_outputs[1];
    **persistent[0] += 1;
    return outputs[1] == &first && *outputs[2] == &first_value
        && **outputs[1] == 7 && int_second_value == 6;
}

static char char_first_value = 'a';
static char char_second_value = 'b';
static char *char_first = &char_first_value;
static char *char_second = &char_second_value;
static char **char_global_outputs[2] = { &char_first, &char_second };

static void set_char(char **output, char *value) { *output = value; }
static int check_char(void) {
    char first_value = 'c';
    char second_value = 'd';
    char *first = &first_value;
    char *second = &second_value;
    char **outputs[3] = { &first, [2] = &second };
    static char **persistent[2] = { &char_first, &char_second };
    if (sizeof(outputs) != 3 * sizeof(outputs[0])) return 0;
    if (outputs[0] != &first || outputs[1] != 0 || outputs[2] != &second) return 0;
    outputs[1] = outputs[0];
    set_char(outputs[2], &first_value);
    persistent[0] = char_global_outputs[1];
    **persistent[0] = 'e';
    return outputs[1] == &first && *outputs[2] == &first_value
        && **outputs[1] == 'c' && char_second_value == 'e';
}

static _Bool bool_first_value = 0;
static _Bool bool_second_value = 1;
static _Bool *bool_first = &bool_first_value;
static _Bool *bool_second = &bool_second_value;
static _Bool **bool_global_outputs[2] = { &bool_first, &bool_second };

static void set_bool(_Bool **output, _Bool *value) { *output = value; }
static int check_bool(void) {
    _Bool first_value = 0;
    _Bool second_value = 1;
    _Bool *first = &first_value;
    _Bool *second = &second_value;
    _Bool **outputs[3] = { &first, [2] = &second };
    static _Bool **persistent[2] = { &bool_first, &bool_second };
    if (sizeof(outputs) != 3 * sizeof(outputs[0])) return 0;
    if (outputs[0] != &first || outputs[1] != 0 || outputs[2] != &second) return 0;
    outputs[1] = outputs[0];
    set_bool(outputs[2], &first_value);
    persistent[0] = bool_global_outputs[1];
    **persistent[0] = 0;
    return outputs[1] == &first && *outputs[2] == &first_value
        && **outputs[1] == 0 && bool_second_value == 0;
}

static double double_first_value = 1.25;
static double double_second_value = 2.5;
static double *double_first = &double_first_value;
static double *double_second = &double_second_value;
static double **double_global_outputs[2] = { &double_first, &double_second };

static void set_double(double **output, double *value) { *output = value; }
static int check_double(void) {
    double first_value = 3.75;
    double second_value = 4.5;
    double *first = &first_value;
    double *second = &second_value;
    double **outputs[3] = { &first, [2] = &second };
    static double **persistent[2] = { &double_first, &double_second };
    if (sizeof(outputs) != 3 * sizeof(outputs[0])) return 0;
    if (outputs[0] != &first || outputs[1] != 0 || outputs[2] != &second) return 0;
    outputs[1] = outputs[0];
    set_double(outputs[2], &first_value);
    persistent[0] = double_global_outputs[1];
    **persistent[0] = 6.25;
    return outputs[1] == &first && *outputs[2] == &first_value
        && **outputs[1] == 3.75 && double_second_value == 6.25;
}

int main(void) {
    return check_int() && check_char() && check_bool() && check_double() ? 0 : 1;
}
