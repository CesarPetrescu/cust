int main(void) {
    char *middle = &"cast"[2];
    char *reverse = &1["dog"];
    char *grouped = &("hi")[1];

    if (middle[0] != 's') {
        return 1;
    }
    if (*reverse != 'o') {
        return 2;
    }
    if (grouped[0] != 'i') {
        return 3;
    }
    if (middle[-1] != 'a') {
        return 4;
    }

    return 0;
}
