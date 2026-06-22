int use_static_enum(int seed) {
    static enum { SAVED = 4 } saved = SAVED;
    saved += seed;
    return saved;
}

int main(void) {
    int total = 0;
    for (enum { START = 2, STOP = 5 } i = START; i < STOP; i = i + 1) {
        total += i;
    }

    auto enum { AUTO_VALUE = 6 } auto_value = AUTO_VALUE;
    register enum { REGISTER_VALUE = 7 } register_value = REGISTER_VALUE;
    total += auto_value + register_value;

    total += use_static_enum(3);
    total += use_static_enum(5);
    return total;
}
