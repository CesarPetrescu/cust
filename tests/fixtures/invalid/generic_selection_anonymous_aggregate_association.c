int main(void) {
    return _Generic(1, struct { int value; }: 2, default: 3);
}
