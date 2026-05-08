int sum_text(char text[5]) {
    return text[0] + text[1] + text[2] + text[3] + text[4];
}

int main(void) {
    char word[4] = "cat";
    char padded[5] = "hi";
    const char read_only[3] = "xy";
    static char saved[4] = "A\x2a";

    int total = 0;
    total += word[0] == 'c';
    total += word[3] == 0;
    total += padded[2] == 0;
    total += read_only[1] == 'y';
    saved[0] = saved[0] + 1;
    total += saved[0] == 'B';
    total += saved[1] == 42;
    total += sum_text(padded) == ('h' + 'i');

    return total;
}
