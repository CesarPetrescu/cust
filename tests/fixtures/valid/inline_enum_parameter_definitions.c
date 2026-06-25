int add_mode(enum Mode { MODE_READY = 3, MODE_BUSY = 5 } mode) {
    return mode + MODE_READY + MODE_BUSY;
}

int main(void) {
    return add_mode(4);
}
