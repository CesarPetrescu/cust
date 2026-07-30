unsigned long int strlen(const char *text);

char *mark(int *calls, char *text) {
    *calls += 1;
    return text;
}

int main(void) {
    int calls = 0;
    char embedded_nul[5] = {'a', 'b', 0, 'c', 0};

    if (strlen("") != 0 || strlen("hello") != 5) {
        return 1;
    }
    if (strlen("xxabc" + 2) != 3 || strlen(embedded_nul) != 2) {
        return 2;
    }
    if (strlen(mark(&calls, "once")) != 4 || calls != 1) {
        return 3;
    }
    if (sizeof(strlen(mark(&calls, "unevaluated"))) != sizeof(unsigned long int)
            || calls != 1) {
        return 4;
    }

    return 0;
}