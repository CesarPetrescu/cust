struct Pair {
    int value;
};

static int global_value = 7;
static int other_value = 9;
static void *global_pointer = &global_value;
static const void *global_const_pointer = &global_value;

static int accepts_void_pointers(void *pointer, const void *const_pointer) {
    int *value = pointer;
    const int *const_value = const_pointer;
    return pointer == &global_value && const_pointer == &global_value &&
           value == &global_value && const_value == &global_value &&
           *value == 7 && *const_value == 7;
}

static int uses_block_static(int *source) {
    static void *saved;
    if (!saved) {
        saved = source;
    }
    return *(int *)saved;
}

int main(void) {
    int local = 11;
    const int fixed = 13;
    struct Pair pair = {17};
    void *local_pointer = 0;
    const void *local_const_pointer = 0;
    int marker = 0;

    void *selected = (marker++, marker == 1 ? &local : global_pointer);
    void *assigned = (marker++, local_pointer = marker == 2 ? selected : global_pointer);
    local_const_pointer = (marker++, marker == 3 ? &fixed : global_const_pointer);

    int *roundtrip = assigned;
    const int *const_roundtrip = local_const_pointer;
    struct Pair *pair_roundtrip = (void *)&pair;
    *roundtrip = 12;

    if (marker != 3 || local != 12 || *const_roundtrip != 13 ||
        pair_roundtrip != &pair || pair_roundtrip->value != 17) {
        return 1;
    }
    if (!assigned || assigned != &local || &local != assigned ||
        assigned == global_pointer || global_pointer != &global_value) {
        return 2;
    }
    if (!accepts_void_pointers(global_pointer, global_const_pointer) ||
        uses_block_static(&other_value) != 9) {
        return 3;
    }
    if (_Generic(assigned, void *: 1, default: 0) != 1 ||
        _Generic(local_const_pointer, const void *: 1, default: 0) != 1) {
        return 4;
    }
    if (sizeof(void *) != sizeof(int *) || _Alignof(void *) != _Alignof(int *) ||
        sizeof(assigned) != sizeof(roundtrip) ||
        sizeof((marker++, assigned)) != sizeof(void *) || marker != 3) {
        return 5;
    }

    local_pointer = 0;
    if (local_pointer || local_pointer != 0) {
        return 6;
    }
    return 0;
}
