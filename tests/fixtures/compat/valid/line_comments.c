int helper(int value) {
    // Line comments are whitespace before a normal statement.
    return value + 2; // trailing line comments stop before the newline
}

int main(void) {
    int value = 3; // declaration tail comment
    char slash = '/';
    char *text = "// not a comment";
    // The next line proves comments do not consume the newline or following statement.
    value = helper(value);
    return value + (slash == '/') + (text[0] == '/') + (text[1] == '/');
}
