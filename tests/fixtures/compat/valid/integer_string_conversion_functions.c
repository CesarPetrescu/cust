int atoi(const char *text);
long int atol(const char *text);
long long int atoll(const char *text);

char *mark(int *counter, char *text) {
    *counter += 1;
    return text;
}

int main(void) {
    int counter = 0;

    if (atoi(" \t\n\r\v\f-123tail") != -123) {
        return 1;
    }
    if (atol("+45") != 45) {
        return 2;
    }
    if (atoll("xyz") != 0 || atoi("") != 0 || atol("   -") != 0) {
        return 3;
    }
    if (atoi("xx-8" + 2) != -8) {
        return 4;
    }
    if (atoll(mark(&counter, " 007x")) != 7 || counter != 1) {
        return 5;
    }
    if (sizeof(atoi(mark(&counter, "9"))) != sizeof(int) || counter != 1) {
        return 6;
    }

    return 0;
}
