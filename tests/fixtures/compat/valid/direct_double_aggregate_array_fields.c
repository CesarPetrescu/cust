struct Measurements {
    double values[4];
    int marker;
};

struct Envelope {
    struct Measurements readings;
};

union DoubleBuffer {
    double values[2];
};

struct Measurements bump(struct Measurements input) {
    input.values[0] += 0.5;
    input.values[3] = 9.0;
    input.marker += 1;
    return input;
}

int index_calls = 0;
int mark_calls = 0;

int next_index(void) {
    index_calls++;
    return 0;
}

int mark(void) {
    mark_calls++;
    return 0;
}

int main(void) {
    struct Measurements direct = {.values = {1.25, [2] = 3.5}, .marker = 7};
    struct Measurements indirect = {{4.0, 5.0, 6.0, 7.0}, 8};
    struct Measurements *slot = &indirect;
    struct Envelope nested = {{{2.0, 3.0, 4.0, 5.0}, 9}};
    struct Envelope *nested_pointer = &nested;
    struct Measurements objects[1] = {{{6.0, 7.0, 8.0, 9.0}, 10}};
    struct Measurements source = {{10.0, 11.0, 12.0, 13.0}, 11};
    struct Measurements initialized_copy = source;
    struct Measurements assigned_copy = {{0.0}, 0};
    assigned_copy = source;
    struct Measurements returned = bump(source);
    const struct Measurements fixed = {{1.0, 2.0, 3.0, 4.0}, 12};
    union DoubleBuffer buffer = {.values = {2.25, 4.5}};
    struct Measurements wrapper_copy = {{0.0}, 0};

    double assigned = (direct.values[0] = 2);
    double compounded = (slot->values[1] += 0.5);
    double post = nested.readings.values[2]++;
    double pre = --objects[0].values[3];

    initialized_copy.values[0] = 20.0;
    assigned_copy.values[1] = 21.0;
    buffer.values[1] += 0.5;
    (_Generic(0, int: objects[0])).values[0] = 26.0;
    (_Generic(0, int: nested.readings)).values[0] += 3.0;
    (_Generic(0, int: nested_pointer->readings)).values[1]++;
    (_Generic(0, int: *slot)).values[2] = 28.0;

    double reversed = next_index()[direct.values];
    double sequenced = (mark(), objects[0].values)[1];
    double selected = (1 ? slot->values : slot->values)[0];
    double conditional = (1 ? direct : source).values[2];
    double comma = (mark(), (struct Measurements){{14.0, 15.0, 16.0, 17.0}, 0}).values[1];
    double wrapper_assigned = (wrapper_copy = direct).values[0];

    if (direct.values[1] != 0.0) return 1;
    if (direct.values[2] != 3.5) return 15;
    if (assigned != 2.0) return 2;
    if (compounded != 5.5) return 16;
    if (post != 4.0) return 17;
    if (pre != 8.0) return 18;
    if (direct.values[0] != 2.0 || indirect.values[1] != 5.5) return 3;
    if (nested.readings.values[2] != 5.0 || objects[0].values[3] != 8.0) return 4;
    if (source.values[0] != 10.0 || source.values[1] != 11.0) return 5;
    if (initialized_copy.values[0] != 20.0 || initialized_copy.values[1] != 11.0) return 6;
    if (assigned_copy.values[0] != 10.0 || assigned_copy.values[1] != 21.0) return 7;
    if (returned.values[0] != 10.5 || returned.values[3] != 9.0 || returned.marker != 12) return 8;
    if (fixed.values[2] != 3.0) return 9;
    if (buffer.values[0] != 2.25 || buffer.values[1] != 5.0) return 19;
    if (reversed != 2.0 || index_calls != 1) return 20;
    if (sequenced != 7.0 || selected != 4.0 || mark_calls != 2) return 21;
    if (conditional != 3.5 || comma != 15.0 || wrapper_assigned != 2.0) return 22;
    if (sizeof(1 ? slot->values[0] : direct.values[0]) != sizeof(double)) return 23;
    if (!_Generic((1 ? slot->values[0] : direct.values[0]), double: 1, default: 0)) return 24;
    if (slot->values[0] != 4.0) return 25;
    if (objects[0].values[0] != 26.0 || nested.readings.values[0] != 5.0) return 26;
    if (nested.readings.values[1] != 4.0 || indirect.values[2] != 28.0) return 27;
    if (!_Generic(direct.values[0], double: 1, default: 0)) return 10;
    if (sizeof(direct.values) != 4 * sizeof(double)) return 11;
    if (sizeof(slot->values) != 4 * sizeof(double)) return 12;
    if (sizeof(nested.readings.values[0]) != sizeof(double)) return 13;
    if (sizeof(objects[0].values) != 4 * sizeof(double)) return 14;
    return 0;
}
