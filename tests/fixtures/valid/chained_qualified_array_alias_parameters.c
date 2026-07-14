struct Point {
    int x;
    int y;
};

enum State {
    IDLE = 2,
    READY = 5
};

typedef const int ConstInt;
typedef const struct Point ConstPoint;
typedef const enum State ConstState;

typedef ConstInt ConstInts[3], ConstIntPair[2], *ConstIntView;
typedef ConstPoint ConstPoints[2], ConstPointTriplet[3], *ConstPointView;
typedef ConstState ConstStates[2], ConstStateTriplet[3], *ConstStateView;

typedef ConstInts ChainedConstInts;
typedef ConstPoints ChainedConstPoints;
typedef ConstStates ChainedConstStates;

ChainedConstInts global_ints = {1, 2, 3};
ChainedConstPoints global_points = {{4, 5}, {6, 7}};
ChainedConstStates global_states = {IDLE, READY};

int inspect(
    ChainedConstInts,
    ConstIntView,
    ChainedConstPoints,
    ConstPointView,
    ChainedConstStates,
    ConstStateView
);

int inspect(
    ChainedConstInts ints,
    ConstIntView expected_ints,
    ChainedConstPoints points,
    ConstPointView expected_points,
    ChainedConstStates states,
    ConstStateView expected_states
) {
    ints = expected_ints;
    points = expected_points;
    states = expected_states;
    return ints == expected_ints
        && points == expected_points
        && states == expected_states
        && ints[2] == expected_ints[2]
        && points[1].y == expected_points[1].y
        && states[1] == expected_states[1];
}

int main(void) {
    int shadow_ok = 0;
    static ChainedConstInts local_ints = {8, 9, 10};
    static ChainedConstPoints local_points = {{11, 12}, {13, 14}};
    static ChainedConstStates local_states = {READY, IDLE};
    ConstIntView int_view = local_ints + 1;
    ConstPointView point_view = local_points + 1;
    ConstStateView state_view = local_states + 1;
    ConstIntPair pair = {15, 16};
    ConstPointTriplet point_triplet = {{17, 18}, {19, 20}, {21, 22}};
    ConstStateTriplet state_triplet = {IDLE, READY, IDLE};

    {
        typedef int ChainedConstInts;
        typedef char ChainedConstPoints;
        typedef int ChainedConstStates;
        ChainedConstInts ints = 3;
        ChainedConstPoints points = 4;
        ChainedConstStates states = 5;
        shadow_ok = ints + points + states == 12;
    }

    return inspect(
        global_ints,
        global_ints,
        global_points,
        global_points,
        global_states,
        global_states
    )
        + 2 * inspect(
            local_ints,
            local_ints,
            local_points,
            local_points,
            local_states,
            local_states
        )
        + 4 * (sizeof(ChainedConstInts) == 3 * sizeof(int)
            && sizeof(ChainedConstPoints) == 2 * sizeof(struct Point)
            && sizeof(ChainedConstStates) == 2 * sizeof(enum State))
        + 8 * (_Alignof(ChainedConstInts) == _Alignof(int)
            && _Alignof(ChainedConstPoints) == _Alignof(struct Point)
            && _Alignof(ChainedConstStates) == _Alignof(enum State))
        + 16 * (int_view == local_ints + 1
            && point_view == local_points + 1
            && state_view == local_states + 1)
        + 32 * shadow_ok
        + 64 * (pair[1] == 16
            && point_triplet[2].y == 22
            && state_triplet[1] == READY);
}
