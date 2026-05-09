int sum_text(char text[]) {
    return text[0] + text[1] + text[2] + text[3] + text[4];
}

int main(void) {
    char word[6] = "he" "llo";
    char *tail = "ab" "cd" + 2;
    int size = sizeof("xy" "z");
    int from_call = sum_text("hi" "!\0");
    char *bytes = (char[]){"A" "B"};
    return (word[0] + word[4] + tail[0] + tail[1] + size + from_call + bytes[0] + bytes[1]) % 256;
}
