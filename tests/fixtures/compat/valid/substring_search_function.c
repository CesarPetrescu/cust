char *strstr(const char *haystack, const char *needle);

char *mark_haystack(int *calls, char *haystack) {
    *calls += 1;
    return haystack;
}

char *mark_needle(int *calls, char *needle) {
    *calls += 1;
    return needle;
}

int main(void) {
    int haystack_calls = 0;
    int needle_calls = 0;
    char text[12] = "abracadabra";
    char needle[5] = "cada";
    char *first = strstr(text, needle);

    if (first != &text[4] || strstr(text, "abra") != text
            || strstr(text + 1, "abra") != &text[7]) {
        return 1;
    }
    if (strstr(text, "xyz") != 0 || strstr(text, "abracadabrax") != 0) {
        return 2;
    }
    if (strstr(text, "") != text || strstr(text + 3, "") != &text[3]
            || strstr("", "") == 0) {
        return 3;
    }
    if (strstr(mark_haystack(&haystack_calls, text),
               mark_needle(&needle_calls, needle)) != &text[4]
            || haystack_calls != 1 || needle_calls != 1) {
        return 4;
    }
    *first = 'C';
    if (text[4] != 'C') {
        return 5;
    }
    if (sizeof(strstr(mark_haystack(&haystack_calls, text),
                      mark_needle(&needle_calls, needle))) != sizeof(char *)
            || sizeof(*strstr(mark_haystack(&haystack_calls, text),
                              mark_needle(&needle_calls, needle))) != sizeof(char)
            || haystack_calls != 1 || needle_calls != 1) {
        return 6;
    }
    if (!strstr(text, "bra") || strstr(text, "xyz")) {
        return 7;
    }
    if (strstr(text, "bra") + 1 != &text[2]) {
        return 8;
    }

    return 0;
}
