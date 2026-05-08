int main(void) {
    char octal = '\101';
    char hex = '\x2a';
    char *text = "\12\x07";
    return octal + hex + text[0] + text[1] + text[2];
}
