int main(void) {
    char *middle = &"cat"[1];
    middle[0] = 'u';
    return middle[0];
}
