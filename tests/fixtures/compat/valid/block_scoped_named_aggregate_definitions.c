int main(void) {
    struct Pair {
        int x;
        int y;
    };
    struct Pair p = {3, 4};

    int total = p.x * 10 + p.y;
    {
        union Number {
            int value;
            char tag;
        };
        union Number n = {5};
        total = total + n.value;
    }

    struct Flags {
        enum State {
            READY = 7,
            DONE = 11
        } state;
    };
    struct Flags flags = {DONE};
    return total + READY + flags.state;
}
