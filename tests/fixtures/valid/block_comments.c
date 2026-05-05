/* leading file comment before the first token */
int main() {
    int total = 1;
    /* multi-line comments should behave like whitespace:
       they may include token-looking text such as return 99; { } && ||
       and the lexer should resume on the line after the closing marker. */
    total += 2;
    int compact = 4/* inline block comment */+ 5;
    char slash = '/';
    char star = '*';
    return total + compact + (slash == 47) + (star == 42);
}
