int main(void) {
    _Atomic(struct { int value; }) left;
    _Atomic(struct { int value; }) right;
    left = right;
    return 0;
}
