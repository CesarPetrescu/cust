enum State {
    READY = 3,
    BUSY = 7
};

typedef enum State State;
typedef const enum State ConstState;
typedef _Atomic(State) AtomicState;

_Atomic(enum State) global = READY;

struct Holder {
    _Atomic(enum State) direct;
    _Atomic(State) alias;
    AtomicState named;
};

int score(_Atomic(State) value, AtomicState other) {
    return value + other;
}

int main(void) {
    struct Holder holder = {READY, BUSY, READY};
    _Atomic(State) local = BUSY;

    typedef enum State ConstState;
    _Atomic(ConstState) shadowed = READY;

    return global + holder.direct + holder.alias + holder.named
        + score(local, shadowed)
        + (sizeof(holder.direct) == sizeof(AtomicState))
        + (_Alignof(_Atomic(enum State)) == _Alignof(AtomicState));
}
