int main(void) {
    int char_sum = '\a' + '\b' + '\f' + '\r' + '\v' + '\?';
    char *text = "\a\b\f\r\v\?";
    int string_sum = text[0] + text[1] + text[2] + text[3] + text[4] + text[5] + text[6];
    return char_sum + string_sum;
}
