struct GlobalPoint {
    int x;
    int y;
} global_point = {2, 3}, *global_slot = &global_point;

int bump_static(void) {
    static struct Counter {
        int value;
    } counter = {4};
    counter.value = counter.value + 1;
    return counter.value;
}

int main(void) {
    struct LocalPair {
        int x;
        int y;
    } pair = {5, 6}, copy = pair, *slot = &pair;

    union LocalNumber {
        int value;
        char tag;
    } number = {7}, *number_slot = &number;

    struct Flags {
        enum State {
            READY = 11,
            DONE = 13
        } state;
    } flags = {DONE};

    slot->x = slot->x + global_slot->y;
    number_slot->value = number_slot->value + READY;

    return global_point.x + pair.x + copy.y + number.value + flags.state + bump_static() + bump_static();
}
