int *bad(char *text) {
    return text;
}

int main() {
    char text[2] = {'x', 0};
    int *value = bad(text);
    return *value;
}
