int main(void) {
    const enum { LOCKED = 3 } value = LOCKED;
    value = 4;
    return value;
}
