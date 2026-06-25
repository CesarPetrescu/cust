enum Outer {
    OUTER_READY = 3,
};

enum Status { STATUS_READY = 5, STATUS_BUSY = 7 } choose_status(int flag);

enum Status choose_status(int flag) {
    if (flag) {
        return STATUS_BUSY;
    }
    return STATUS_READY;
}

enum Mode { MODE_FAST = 11 } (choose_mode)(void) {
    return MODE_FAST;
}

int main(void) {
    enum Status status = choose_status(1);
    enum Mode mode = choose_mode();
    return OUTER_READY + STATUS_READY + status + MODE_FAST + mode;
}
