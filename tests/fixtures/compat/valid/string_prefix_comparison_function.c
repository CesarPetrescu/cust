int strncmp(const char *left, const char *right, unsigned long int count);

char *mark_text(int *calls, char *text) {
    *calls += 1;
    return text;
}

unsigned long int mark_count(int *calls, unsigned long int count) {
    *calls += 1;
    return count;
}

int main(void) {
    int left_calls = 0;
    int right_calls = 0;
    int count_calls = 0;
    char left_prefix[2] = {'a', 'b'};
    char right_prefix[2] = {'a', 'b'};
    char embedded_left[4] = {'a', 0, 'z', 0};
    char embedded_right[4] = {'a', 0, 'y', 0};

    if (strncmp("same", "same", 4) != 0
            || strncmp("alphabet", "alpine", 2) != 0) {
        return 1;
    }
    if (strncmp("alpha", "alpine", 4) >= 0
            || strncmp("zeta", "beta", 1) <= 0) {
        return 2;
    }
    if (strncmp("left", "right", 0) != 0
            || strncmp(embedded_left, embedded_right, 4) != 0) {
        return 3;
    }
    if (strncmp("xxalpha" + 2, "alpha", 5) != 0
            || strncmp("\x80", "\x7f", 1) <= 0) {
        return 4;
    }
    if (strncmp(left_prefix, right_prefix, 2) != 0) {
        return 5;
    }
    if (strncmp(mark_text(&left_calls, "once"),
                mark_text(&right_calls, "only"),
                mark_count(&count_calls, 1)) != 0
            || left_calls != 1 || right_calls != 1 || count_calls != 1) {
        return 6;
    }
    if (sizeof(strncmp(mark_text(&left_calls, "left"),
                       mark_text(&right_calls, "right"),
                       mark_count(&count_calls, 3))) != sizeof(int)
            || left_calls != 1 || right_calls != 1 || count_calls != 1) {
        return 7;
    }

    return 0;
}
