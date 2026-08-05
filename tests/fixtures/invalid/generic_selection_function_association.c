int main(void) {
    return _Generic(1, int(void): 2, default: 3);
}
