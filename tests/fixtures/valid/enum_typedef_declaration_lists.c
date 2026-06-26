typedef enum State {
    STATE_IDLE = 2,
    STATE_BUSY = 5,
    STATE_DONE = 8,
} State, *StatePtr, StateArray[4];

int sum_states(StateArray states, int len) {
    StatePtr cursor = states;
    int total = 0;
    for (int i = 0; i < len; i = i + 1) {
        total = total + cursor[i];
    }
    return total;
}

int bump_second(StatePtr states) {
    states[1] = (State){STATE_DONE};
    return states[1];
}

int main(void) {
    StateArray states = {STATE_IDLE, STATE_BUSY, [3] = STATE_DONE};
    StatePtr tail = states + 2;
    tail[0] = (State)(STATE_BUSY + 1);
    bump_second(states);

    return sum_states(states, 4)
        + tail[-1]
        + sizeof(StateArray) / sizeof(State)
        + (_Alignof(StateArray) == _Alignof(State))
        + sizeof(StatePtr) / sizeof(StatePtr);
}
