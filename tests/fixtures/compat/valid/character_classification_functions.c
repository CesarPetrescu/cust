int isalnum(int value);
int isalpha(int value);
int isblank(int value);
int iscntrl(int value);
int isdigit(int value);
int isgraph(int value);
int islower(int value);
int isprint(int value);
int ispunct(int value);
int isspace(int value);
int isupper(int value);
int isxdigit(int value);

int marker = 0;

int mark(int value) {
    marker = marker + 1;
    return value;
}

int main(void) {
    if (!isalnum(mark('A')) || !isalnum(mark('7')) || isalnum(mark('!'))) {
        return 1;
    }
    if (!isalpha(mark('Z')) || !isalpha(mark('a')) || isalpha(mark('4'))) {
        return 2;
    }
    if (!isblank(mark(' ')) || !isblank(mark('\t')) || isblank(mark('\n'))) {
        return 3;
    }
    if (!iscntrl(mark(0)) || !iscntrl(mark(127)) || iscntrl(mark(' '))) {
        return 4;
    }
    if (!isdigit(mark('0')) || !isdigit(mark('9')) || isdigit(mark('A'))) {
        return 5;
    }
    if (!isgraph(mark('!')) || !isgraph(mark('~')) || isgraph(mark(' '))) {
        return 6;
    }
    if (!islower(mark('a')) || !islower(mark('z')) || islower(mark('A'))) {
        return 7;
    }
    if (!isprint(mark(' ')) || !isprint(mark('~')) || isprint(mark('\n'))) {
        return 8;
    }
    if (!ispunct(mark('!')) || !ispunct(mark('@')) || ispunct(mark('a'))) {
        return 9;
    }
    if (!isspace(mark(' ')) || !isspace(mark('\f')) || !isspace(mark('\n'))
            || !isspace(mark('\r')) || !isspace(mark('\t')) || !isspace(mark('\v'))
            || isspace(mark('A'))) {
        return 10;
    }
    if (!isupper(mark('A')) || !isupper(mark('Z')) || isupper(mark('a'))) {
        return 11;
    }
    if (!isxdigit(mark('0')) || !isxdigit(mark('9')) || !isxdigit(mark('a'))
            || !isxdigit(mark('f')) || !isxdigit(mark('A')) || !isxdigit(mark('F'))
            || isxdigit(mark('g'))) {
        return 12;
    }
    if (isalnum(-1) || isalpha(255) || iscntrl(255) || isprint(255)) {
        return 13;
    }

    int before_sizeof = marker;
    if (sizeof(isalpha(mark('Q'))) != sizeof(int) || marker != before_sizeof) {
        return 14;
    }

    return marker == 44 ? 0 : 15;
}
