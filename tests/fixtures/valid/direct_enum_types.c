enum State {
    READY = 2,
    BUSY,
};

enum State global_state = READY;
const enum State const_state = BUSY;

enum State choose(enum State left, enum State right) {
    if (left == READY) {
        return right;
    }
    return left;
}

int score(enum State state);

int score(enum State state) {
    return state + sizeof(enum State) + sizeof(const enum State);
}

int main(void) {
    enum State local = choose(global_state, const_state);
    int values[2] = {READY, local};
    return score(local) + values[0] + values[1];
}
