unsigned long int strspn(const char *text, const char *accept);
unsigned long int strcspn(const char *text, const char *reject);

char *mark(int *calls, char *text) {
    *calls += 1;
    return text;
}

int main(void) {
    int text_calls = 0;
    int set_calls = 0;
    char text[9] = "abcde123";
    char accept[4] = "cba";
    char reject[3] = "1x";

    if (strspn(text, accept) != 3 || strcspn(text, reject) != 5) {
        return 1;
    }
    if (strspn("", accept) != 0 || strspn(text, "") != 0
            || strcspn("", reject) != 0 || strcspn(text, "") != 8) {
        return 2;
    }
    if (strspn(text + 2, "cde") != 3 || strcspn(text + 2, "1") != 3) {
        return 3;
    }
    if (strspn(mark(&text_calls, text), mark(&set_calls, accept)) != 3
            || text_calls != 1 || set_calls != 1) {
        return 4;
    }
    if (strcspn(mark(&text_calls, text), mark(&set_calls, reject)) != 5
            || text_calls != 2 || set_calls != 2) {
        return 5;
    }
    if (sizeof(strspn(mark(&text_calls, text), mark(&set_calls, accept)))
                != sizeof(unsigned long int)
            || sizeof(strcspn(mark(&text_calls, text), mark(&set_calls, reject)))
                != sizeof(unsigned long int)
            || text_calls != 2 || set_calls != 2) {
        return 6;
    }

    return 0;
}
