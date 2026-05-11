typedef int Count;
typedef char Small;

enum Mode {
    MODE_IDLE = 1,
    MODE_BUSY,
    MODE_DONE = MODE_BUSY + 3,
    MODE_EXTRA = (MODE_DONE * 2) - 1,
    MODE_CMP = (MODE_EXTRA > MODE_DONE) + (MODE_BUSY == 2) + (MODE_IDLE != 0),
    MODE_LOGIC = (MODE_CMP == 3) && (MODE_DONE >= 5) ? 11 : 12,
    MODE_COND = MODE_LOGIC == 11 ? MODE_EXTRA + 2 : 0,
    MODE_SIZE = sizeof(char[5]) + _Alignof(char),
    MODE_CAST = (Count)(MODE_SIZE + (Small)6)
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
    case MODE_CMP:
        return 60;
    case MODE_COND:
        return 70;
    case MODE_SIZE:
        return 6;
    case MODE_CAST:
        return 12;
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
    case (LOCAL_BASE <= LOCAL_NEXT) && (LOCAL_NEXT >= LOCAL_BASE) ? 8 : 9:
        return 8;
    case sizeof(char[3]) + _Alignof(char):
        return 9;
    case (Count)(LOCAL_BASE + (Small)5):
        return 6;
    default:
        return 3;
    }
}

int main(void) {
    return classify(MODE_DONE) + block_scoped_case(7) + classify(99)
        + classify(MODE_EXTRA) + block_scoped_case(27) + block_scoped_case(1)
        + block_scoped_case(10) + classify(MODE_COND) + block_scoped_case(8)
        + classify(MODE_SIZE) + block_scoped_case(4) + classify(MODE_CAST)
        + block_scoped_case(11);
}
