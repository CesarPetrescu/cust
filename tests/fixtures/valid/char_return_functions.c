char first_char(char *text);
char bump(char value) {
    return value + 1;
}

char first_char(char *text) {
    return text[0];
}

int main() {
    char letter = bump('A');
    char first = first_char("cat");
    return (letter == 'B') + (first == 'c') + sizeof(first_char("z"));
}
