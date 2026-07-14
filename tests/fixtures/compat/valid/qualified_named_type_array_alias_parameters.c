struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

enum State {
    IDLE = 2,
    READY = 5
};

typedef const struct Point ConstPoint;
typedef const union Number ConstNumber;
typedef const enum State ConstState;

typedef ConstPoint ConstPoints[2];
typedef ConstNumber ConstNumbers[2];
typedef ConstState ConstStates[2];

ConstPoints global_points = {{1, 2}, {3, 4}};
ConstNumbers global_numbers = {{5}, {7}};
ConstStates global_states = {IDLE, READY};

int inspect(
    ConstPoints,
    ConstPoint *,
    ConstNumbers,
    ConstNumber *,
    ConstStates,
    ConstState *
);

int inspect(
    ConstPoints points,
    ConstPoint *expected_points,
    ConstNumbers numbers,
    ConstNumber *expected_numbers,
    ConstStates states,
    ConstState *expected_states
) {
    points = expected_points;
    numbers = expected_numbers;
    states = expected_states;
    return points == expected_points
        && numbers == expected_numbers
        && states == expected_states
        && points[1].y == expected_points[1].y
        && numbers[1].value == expected_numbers[1].value
        && states[1] == expected_states[1];
}

int main(void) {
    int shadow_ok = 0;
    static ConstPoints local_points = {{6, 7}, {8, 9}};
    static ConstNumbers local_numbers = {{10}, {11}};
    static ConstStates local_states = {READY, IDLE};
    ConstPoint *point_view = local_points + 1;
    ConstNumber *number_view = local_numbers;
    ConstState *state_view = local_states + 1;

    {
        typedef int ConstPoints;
        typedef char ConstNumbers;
        typedef int ConstStates;
        ConstPoints point = 3;
        ConstNumbers number = 4;
        ConstStates state = 5;
        shadow_ok = point + number + state == 12;
    }

    return inspect(
        global_points,
        global_points,
        global_numbers,
        global_numbers,
        global_states,
        global_states
    )
        + 2 * inspect(
            local_points,
            local_points,
            local_numbers,
            local_numbers,
            local_states,
            local_states
        )
        + 4 * (sizeof(ConstPoints) == 2 * sizeof(struct Point)
            && sizeof(ConstNumbers) == 2 * sizeof(union Number)
            && sizeof(ConstStates) == 2 * sizeof(enum State))
        + 8 * (_Alignof(ConstPoints) == _Alignof(struct Point)
            && _Alignof(ConstNumbers) == _Alignof(union Number)
            && _Alignof(ConstStates) == _Alignof(enum State))
        + 16 * (point_view == local_points + 1
            && number_view == local_numbers
            && state_view == local_states + 1)
        + 32 * shadow_ok
        + 64 * (global_points[1].y == 4
            && global_numbers[1].value == 7
            && global_states[1] == READY);
}
