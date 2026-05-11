enum Mode {
    MODE_IDLE = 1,
    MODE_BUSY,
    MODE_DONE = MODE_BUSY + 3,
    MODE_EXTRA = (MODE_DONE * 2) - 1
};

int classify(enum Mode mode) {
    switch (mode) {
    case MODE_IDLE:
        return 10;
    case MODE_BUSY:
        return 20;
    case MODE_DONE:
        return 30;
    case MODE_EXTRA:
        return 50;
    default:
        return 40;
    }
}

int block_scoped_case(int value) {
    enum Local {
        LOCAL_BASE = 6,
        LOCAL_NEXT,
        LOCAL_MASK = (LOCAL_BASE << 2) | 3,
        LOCAL_MIXED = LOCAL_MASK ^ 5,
        LOCAL_HALF = LOCAL_MIXED / 3,
        LOCAL_REM = LOCAL_MIXED % 3
    };

    switch (value) {
    case LOCAL_BASE:
        return 1;
    case LOCAL_NEXT:
        return 2;
    case (LOCAL_BASE << 2) | 3:
        return 4;
    case (LOCAL_MIXED & 10) + LOCAL_REM:
        return 5;
    case !0:
        return 7;
    default:
        return 3;
    }
}

int main(void) {
    return classify(MODE_DONE) + block_scoped_case(7) + classify(99)
        + classify(MODE_EXTRA) + block_scoped_case(27) + block_scoped_case(1)
        + block_scoped_case(10);
}
