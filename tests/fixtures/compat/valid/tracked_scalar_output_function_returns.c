typedef int *IntPtr;
typedef IntPtr *IntOutput;
typedef char *CharPtr;
typedef CharPtr *CharOutput;
typedef _Bool *BoolPtr;
typedef BoolPtr *BoolOutput;
typedef double *DoublePtr;
typedef DoublePtr *DoubleOutput;

int **identity_int(int **output) {
    return output;
}

CharOutput identity_char(CharOutput output) {
    return output;
}

_Bool **identity_bool(_Bool **output) {
    return output;
}

DoubleOutput identity_double(DoubleOutput output) {
    return output;
}

int main(void) {
    int ints[2] = {3, 5};
    char chars[2] = {'a', 'b'};
    _Bool bools[2] = {0, 1};
    double doubles[2] = {1.25, 2.5};
    IntPtr int_slot = ints;
    CharPtr char_slot = chars;
    BoolPtr bool_slot = bools;
    DoublePtr double_slot = doubles;
    IntOutput int_output = identity_int(&int_slot);
    CharOutput char_output = identity_char(&char_slot);
    BoolOutput bool_output = identity_bool(&bool_slot);
    DoubleOutput double_output = identity_double(&double_slot);

    *identity_int(int_output) = ints + 1;
    *identity_char(char_output) = chars + 1;
    *identity_bool(bool_output) = bools + 1;
    *identity_double(double_output) = doubles + 1;

    return int_slot == ints + 1 && char_slot == chars + 1 &&
                   bool_slot == bools + 1 && double_slot == doubles + 1 &&
                   **int_output == 5 && **char_output == 'b' && **bool_output == 1 &&
                   **double_output == 2.5 && sizeof(identity_int(int_output)) == sizeof(int_output) &&
                   sizeof(identity_char(char_output)) == sizeof(char_output) &&
                   sizeof(identity_bool(bool_output)) == sizeof(bool_output) &&
                   sizeof(identity_double(double_output)) == sizeof(double_output)
               ? 0
               : 1;
}
