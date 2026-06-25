enum State {
    READY = 2,
    BUSY = 5,
};

int take(enum State *items, const enum State view[static 2]) {
    return items[1] + view[0];
}

int main(void) {
    enum State values[3] = {READY, (enum State)(READY + BUSY), BUSY};
    enum State *cursor = values;
    const enum State *view = values;
    int relationships = (sizeof(enum State[3]) == 3 * sizeof(enum State))
        + (_Alignof(enum State[2]) == _Alignof(enum State))
        + (sizeof(enum State *) == sizeof(cursor));
    return take(cursor, view)
        + relationships
        + (cursor < &values[2])
        + ((enum State){BUSY} == BUSY);
}
