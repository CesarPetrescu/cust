int score(char text[4]) {
    return text[0] + text[1] + text[2] + text[3];
}

int main() {
    int direct = "abc"[0] + "abc"[1] + "abc"[2] + "abc"[3];
    int escaped = "A\n\0"[0] + "A\n\0"[1] + "A\n\0"[2] + "A\n\0"[3];
    return direct + escaped + score("Hi!" );
}
