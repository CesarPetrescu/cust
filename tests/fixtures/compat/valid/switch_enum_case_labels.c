enum Mode {
    MODE_IDLE = 1,
    MODE_BUSY,
    MODE_DONE = MODE_BUSY + 3
};

int classify(enum Mode mode) {
    switch (mode) {
    case MODE_IDLE:
        return 10;
    case MODE_BUSY:
        return 20;
    case MODE_DONE:
        return 30;
    default:
        return 40;
    }
}

int block_scoped_case(int value) {
    enum Local {
        LOCAL_BASE = 6,
        LOCAL_NEXT
    };

    switch (value) {
    case LOCAL_BASE:
        return 1;
    case LOCAL_NEXT:
        return 2;
    default:
        return 3;
    }
}

int main(void) {
    return classify(MODE_DONE) + block_scoped_case(7) + classify(99);
}
