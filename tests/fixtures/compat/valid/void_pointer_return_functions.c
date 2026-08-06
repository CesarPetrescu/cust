static int global_value = 7;

void *as_void(int *value);
const void *as_const_void(const int *value);
void const *as_postfix_const_void(const int *value);
void volatile *as_postfix_volatile_void(int *value);

void *as_void(int *value) {
    return value;
}

const void *as_const_void(const int *value) {
    return value;
}

void const *as_postfix_const_void(const int *value) {
    return value;
}

void volatile *as_postfix_volatile_void(int *value) {
    return value;
}

void *choose_void(void *left, void *right, int choose_right) {
    return choose_right ? right : left;
}

int *as_int(void *value) {
    return value;
}

int main(void) {
    int local = 11;
    const int fixed = 13;
    int marker = 0;

    void *selected = choose_void(as_void(&global_value), as_void(&local), 1);
    int *roundtrip = as_int((marker++, selected));
    const void *const_result = as_const_void(&fixed);
    const int *const_roundtrip = const_result;
    const int *postfix_const_roundtrip = as_postfix_const_void(&fixed);

    if (marker != 1 || !selected || selected != &local ||
        roundtrip != &local || *roundtrip != 11 ||
        const_roundtrip != &fixed || *const_roundtrip != 13 ||
        postfix_const_roundtrip != &fixed || *postfix_const_roundtrip != 13 ||
        as_postfix_volatile_void(&local) != &local) {
        return 1;
    }
    if (sizeof(as_void((marker++, &local))) != sizeof(void *) || marker != 1) {
        return 2;
    }
    if ((0 ? as_void(&global_value) : as_void(&local)) != &local ||
        (marker++, as_void(&global_value)) != &global_value || marker != 2) {
        return 3;
    }
    return 0;
}
