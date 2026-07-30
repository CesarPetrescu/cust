int strcmp(const char *left, const char *right);

char *mark(int *counter, char *text) {
    *counter += 1;
    return text;
}

int main(void) {
    int left_calls = 0;
    int right_calls = 0;

    if (strcmp("same", "same") != 0) {
        return 1;
    }
    if (strcmp("alpha", "alpine") >= 0 || strcmp("zeta", "beta") <= 0) {
        return 2;
    }
    if (strcmp("xxalpha" + 2, "alpha") != 0) {
        return 3;
    }
    if (strcmp("\x80", "\x7f") <= 0) {
        return 4;
    }
    if (strcmp(mark(&left_calls, "left"), mark(&right_calls, "left")) != 0
        || left_calls != 1 || right_calls != 1) {
        return 5;
    }
    if (sizeof(strcmp(mark(&left_calls, "left"), mark(&right_calls, "right")))
            != sizeof(int)
        || left_calls != 1 || right_calls != 1) {
        return 6;
    }

    return 0;
}