struct Point {
    int x;
    int y;
};

enum State {
    IDLE = 2,
    READY = 5
};

typedef const int ConstInt;
typedef const char ConstChar;
typedef const struct Point ConstPoint;
typedef const enum State ConstState;

typedef ConstInt ConstInts[3];
typedef ConstChar ConstChars[2];
typedef ConstPoint ConstPoints[2];
typedef ConstState ConstStates[2];

typedef ConstInts ChainedConstInts;
typedef ConstChars ChainedConstChars;
typedef ConstPoints ChainedConstPoints;
typedef ConstStates ChainedConstStates;

int inspect(
    const int *ints,
    const struct Point *points,
    const enum State *states
) {
    return ints[0] == 1
        && ints[2] == 3
        && points[0].x == 4
        && points[1].y == 7
        && states[0] == IDLE
        && states[1] == READY;
}

int main(void) {
    int one = 1;
    int shadow_ok = 0;
    const int *ints = (ChainedConstInts){1, 2, 3};
    const struct Point *points = (ChainedConstPoints){{4, 5}, {6, 7}};
    const enum State *states = (ChainedConstStates){IDLE, READY};

    {
        typedef int ChainedConstInts[2];
        typedef struct Point ChainedConstPoints[1];
        typedef enum State ChainedConstStates[2];
        int *mutable_ints = (ChainedConstInts){8, 9};
        struct Point *mutable_points = (ChainedConstPoints){{10, 11}};
        enum State *mutable_states = (ChainedConstStates){IDLE, READY};

        mutable_ints[0] = 12;
        mutable_points[0].y = 13;
        mutable_states[0] = READY;
        shadow_ok = mutable_ints[0] == 12
            && mutable_points[0].y == 13
            && mutable_states[0] == READY;
    }

    return (ints[0] == 1 && points[1].x == 6 && states[1] == READY)
        + 2 * inspect(ints, points, states)
        + 4 * (sizeof((ChainedConstInts){1, 2, 3}) == sizeof(ChainedConstInts)
            && sizeof((ChainedConstChars){'A', 'B'}) == sizeof(ChainedConstChars)
            && sizeof((ChainedConstPoints){{1, 2}, {3, 4}}) == sizeof(ChainedConstPoints)
            && sizeof((ChainedConstStates){IDLE, READY}) == sizeof(ChainedConstStates))
        + 8 * (sizeof(((ChainedConstInts){1, 2, 3})[0]) == sizeof(int)
            && sizeof(((ChainedConstChars){'A', 'B'})[0]) == sizeof(char)
            && sizeof(((ChainedConstPoints){{1, 2}, {3, 4}})[0]) == sizeof(struct Point)
            && sizeof(((ChainedConstStates){IDLE, READY})[0]) == sizeof(enum State))
        + 16 * (ints + one > ints
            && points + one > points
            && states + one > states)
        + 32 * shadow_ok
        + 64 * inspect(
            (ChainedConstInts){1, 2, 3},
            (ChainedConstPoints){{4, 5}, {6, 7}},
            (ChainedConstStates){IDLE, READY}
        );
}