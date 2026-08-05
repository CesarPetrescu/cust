int main(void) {
    char letter = 'x';
    return _Generic(letter, int: 2);
}
