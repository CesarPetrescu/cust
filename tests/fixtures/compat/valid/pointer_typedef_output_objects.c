typedef char *CharPtr;
typedef CharPtr ChainedCharPtr;
typedef int *IntPtr;
typedef _Bool *BoolPtr;
typedef double *DoublePtr;

static CharPtr global_char_slot = 0;
static CharPtr *global_char_output = &global_char_slot;
static IntPtr global_int_slot = 0;
static IntPtr *global_int_output = &global_int_slot;
static BoolPtr global_bool_slot = 0;
static BoolPtr *global_bool_output = &global_bool_slot;
static DoublePtr global_double_slot = 0;
static DoublePtr *global_double_output = &global_double_slot;

static void set_char(ChainedCharPtr *output, CharPtr value) {
    *output = value;
}

static void set_int(int **output, int *value);

static void set_int(IntPtr *output, IntPtr value) {
    *output = value;
}

static void set_bool(BoolPtr *output, BoolPtr value) {
    *output = value;
}

static void set_double(DoublePtr *output, DoublePtr value) {
    *output = value;
}

int main(void) {
    char chars[2] = {1, 2};
    int ints[2] = {3, 4};
    _Bool bools[2] = {0, 0};
    double doubles[2] = {1.25, 2.5};
    CharPtr char_slot = 0, *char_output = &char_slot;
    IntPtr int_slot = 0, *int_output = &int_slot;
    BoolPtr bool_slot = 0, *bool_output = &bool_slot;
    DoublePtr double_slot = 0, *double_output = &double_slot;
    static IntPtr static_slot = 0;
    static IntPtr *static_output = &static_slot;

    set_char(char_output, chars + 1);
    set_int(int_output, ints + 1);
    set_bool(bool_output, bools + 1);
    set_double(double_output, doubles + 1);
    set_char(global_char_output, chars);
    set_int(global_int_output, ints);
    set_bool(global_bool_output, bools);
    set_double(global_double_output, doubles);
    set_int(static_output, ints + 1);

    **char_output = 7;
    **int_output = 9;
    **bool_output = 7;
    **double_output = 9.5;

    if (chars[1] != 7 || ints[1] != 9 || bools[1] != 1 || doubles[1] != 9.5) {
        return 1;
    }
    if (global_char_slot != chars || global_int_slot != ints ||
        global_bool_slot != bools || global_double_slot != doubles ||
        static_slot != ints + 1) {
        return 2;
    }
    return sizeof(char_output) == sizeof(&char_slot) &&
                   sizeof(*int_output) == sizeof(int_slot) &&
                   sizeof(**bool_output) == sizeof(*bool_slot) &&
                   sizeof(**double_output) == sizeof(*double_slot)
               ? 0
               : 3;
}
