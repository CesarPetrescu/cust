int overwrite(char *text) {
    text[0] = 'x';
    return 0;
}

int main() {
    return overwrite("abc");
}
