struct Sample {
    double reading;
    int marker;
};

struct Box {
    struct Sample sample;
};

union Number {
    double real;
    double alternate;
};

struct Sample bump(struct Sample value) {
    value.reading += 0.5;
    value.marker += 1;
    return value;
}

int main(void) {
    struct Sample direct = {1.25, 7};
    struct Sample indirect = {4.75, 9};
    struct Sample *slot = &indirect;
    struct Box box = {{2.5, 11}};
    union Number number = {.real = 4.5};
    struct Sample source = {6.25, 13};
    struct Sample initialized_copy = source;
    struct Sample assigned_copy = {0.0, 0};
    assigned_copy = source;
    initialized_copy.reading = 7.25;
    initialized_copy.marker = 14;
    struct Sample returned = bump(source);

    double assigned = (direct.reading = 2);
    double compounded = (direct.reading += 0.5);
    double post = direct.reading++;
    double pre = --direct.reading;

    slot->reading *= 2.0;
    slot->reading--;
    box.sample.reading /= 2.0;
    number.real += 0.5;

    if (assigned != 2.0 || compounded != 2.5 || post != 2.5 || pre != 2.5) {
        return 1;
    }
    if (direct.reading != 2.5 || direct.marker != 7) {
        return 2;
    }
    if (indirect.reading != 8.5 || indirect.marker != 9) {
        return 3;
    }
    if (box.sample.reading != 1.25 || box.sample.marker != 11) {
        return 4;
    }
    if (number.real != 5.0) {
        return 5;
    }
    if (source.reading != 6.25 || source.marker != 13
        || initialized_copy.reading != 7.25 || initialized_copy.marker != 14
        || assigned_copy.reading != 6.25 || assigned_copy.marker != 13
        || returned.reading != 6.75 || returned.marker != 14) {
        return 6;
    }
    if (sizeof(direct.reading) != sizeof(double)
        || sizeof(slot->reading) != sizeof(double)
        || sizeof(box.sample.reading) != sizeof(double)
        || sizeof(number.real) != sizeof(double)) {
        return 7;
    }
    return 0;
}
