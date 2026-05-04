int identity(char value) {
    return value;
}

int main() {
    char letter = 'A';
    char newline = '\n';
    char quote = '\'';
    char slash = '\\';
    int total = identity(letter);
    total = total + newline;
    total = total + quote;
    total = total + slash;
    return total;
}
