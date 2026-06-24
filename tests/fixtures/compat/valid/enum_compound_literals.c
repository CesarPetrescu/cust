enum State { READY = 4, BUSY = 7 };
typedef enum State State;

int take(enum State value) {
    return value;
}

int main(void) {
    enum State state = (enum State){READY + 2};
    State alias = (State){BUSY};
    int direct = take((enum State){state + 1});
    enum State *values = (enum State[3]){READY, [2] = BUSY};
    return state + alias + direct + values[0] + values[1] + values[2]
        + (sizeof((enum State){READY}) == sizeof(enum State));
}
