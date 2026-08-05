int main(void) {
    return _Generic(1, int: 2, signed int: 3);
}
