int tolower(int value);
int toupper(int value);

int marker = 0;

int mark(int value) {
    marker = marker + 1;
    return value;
}

int main(void) {
    if (tolower(mark('A')) != 'a' || tolower(mark('Z')) != 'z'
            || tolower(mark('a')) != 'a' || tolower(mark('0')) != '0'
            || tolower(mark('!')) != '!') {
        return 1;
    }
    if (toupper(mark('a')) != 'A' || toupper(mark('z')) != 'Z'
            || toupper(mark('A')) != 'A' || toupper(mark('0')) != '0'
            || toupper(mark('!')) != '!') {
        return 2;
    }

    int before_sizeof = marker;
    if (sizeof(tolower(mark('Q'))) != sizeof(int)
            || sizeof(toupper(mark('q'))) != sizeof(int)
            || marker != before_sizeof) {
        return 3;
    }

    return marker == 10 ? 0 : 4;
}
