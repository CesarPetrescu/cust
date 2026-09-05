typedef char *CharPtr;
typedef CharPtr ChainedCharPtr;
typedef CharPtr *CharOutput;
typedef CharOutput ChainedCharOutput;
typedef int *IntPtr;
typedef IntPtr *IntOutput;
typedef _Bool *BoolPtr;
typedef BoolPtr *BoolOutput;
typedef double *DoublePtr;
typedef DoublePtr *DoubleOutput;

static CharPtr global_char_slot = 0;
static ChainedCharOutput global_char_output = &global_char_slot;
static IntPtr global_int_slot = 0;
static IntOutput global_int_output = &global_int_slot;
static BoolPtr global_bool_slot = 0;
static BoolOutput global_bool_output = &global_bool_slot;
static DoublePtr global_double_slot = 0;
static DoubleOutput global_double_output = &global_double_slot;

static void set_char(ChainedCharOutput output, CharPtr value) {
    *output = value;
}

static void set_int(int **output, int *value);

static void set_int(IntOutput output, IntPtr value) {
    *output = value;
}

static void set_bool(BoolOutput output, BoolPtr value) {
    *output = value;
}

static void set_double(DoubleOutput output, DoublePtr value) {
    *output = value;
}

int main(void) {
    char chars[2] = {1, 2};
    int ints[2] = {3, 4};
    _Bool bools[2] = {0, 0};
    double doubles[2] = {1.25, 2.5};
    CharPtr char_slot = 0;
    ChainedCharOutput char_output = &char_slot, char_output_copy = char_output;
    IntPtr int_slot = 0;
    IntOutput int_output = &int_slot, int_output_copy = int_output;
    BoolPtr bool_slot = 0;
    BoolOutput bool_output = &bool_slot, bool_output_copy = bool_output;
    DoublePtr double_slot = 0;
    DoubleOutput double_output = &double_slot, double_output_copy = double_output;
    static IntPtr static_slot = 0;
    static IntOutput static_output = &static_slot;

    set_char(char_output, chars);
    set_int(int_output, ints);
    set_bool(bool_output, bools);
    set_double(double_output, doubles);
    if (char_slot != chars || int_slot != ints || bool_slot != bools ||
        double_slot != doubles) {
        return 1;
    }
    set_char(char_output_copy, chars + 1);
    set_int(int_output_copy, ints + 1);
    set_bool(bool_output_copy, bools + 1);
    set_double(double_output_copy, doubles + 1);
    if (char_slot != chars + 1 || int_slot != ints + 1 || bool_slot != bools + 1 ||
        double_slot != doubles + 1) {
        return 2;
    }
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
        return 3;
    }
    if (global_char_slot != chars || global_int_slot != ints ||
        global_bool_slot != bools || global_double_slot != doubles ||
        static_slot != ints + 1) {
        return 4;
    }
    return sizeof(char_output) == sizeof(&char_slot) &&
                   sizeof(*int_output) == sizeof(int_slot) &&
                   sizeof(**bool_output) == sizeof(*bool_slot) &&
                   sizeof(**double_output) == sizeof(*double_slot)
               ? 0
               : 5;
}
