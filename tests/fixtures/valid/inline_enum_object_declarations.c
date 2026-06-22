enum { GLOBAL_READY = 2, GLOBAL_BUSY = 5 } global_state = GLOBAL_BUSY;
const enum { GLOBAL_LOCKED = 7 } global_const = GLOBAL_LOCKED;
enum Mode { MODE_IDLE = 11, MODE_RUN = 13 } global_mode = MODE_RUN;

int main(void) {
    enum { LOCAL_BASE = GLOBAL_READY + 3, LOCAL_NEXT } local = LOCAL_NEXT;
    enum Mode mode = MODE_IDLE;
    enum { LOCAL_ONE = 1 } first = LOCAL_ONE, second = first + global_state;
    return global_state + global_const + global_mode + local + mode + second;
}
