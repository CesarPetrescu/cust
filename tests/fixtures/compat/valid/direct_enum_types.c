enum State {
    READY = 2,
    BUSY,
};

enum State global_state = READY;

enum State choose(enum State left, enum State right) {
    if (left == READY) {
        return right;
    }
    return left;
}

int score(enum State state);

int score(enum State state) {
    return state + READY;
}

int main(void) {
    enum State local = choose(global_state, BUSY);
    int values[2] = {READY, local};
    return score(local) + values[0] + values[1];
}
