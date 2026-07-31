char *strchr(const char *text, int search);

char *mark_text(int *calls, char *text) {
    *calls += 1;
    return text;
}

int mark_search(int *calls, int search) {
    *calls += 1;
    return search;
}

int main(void) {
    int text_calls = 0;
    int search_calls = 0;
    char text[7] = "banana";
    char *first = strchr(text, 'a');
    char *normalized = strchr(text, 256 + 'n');
    char *terminator = strchr(text, 0);

    if (first != &text[1] || normalized != &text[2]
            || terminator != &text[6] || strchr(text, 'z') != 0) {
        return 1;
    }
    if (strchr(text + 2, 'a') != &text[3]) {
        return 2;
    }
    *first = 'A';
    if (text[1] != 'A') {
        return 3;
    }
    if (strchr(mark_text(&text_calls, text),
               mark_search(&search_calls, 'b')) != text
            || text_calls != 1 || search_calls != 1) {
        return 4;
    }
    if (sizeof(strchr(mark_text(&text_calls, text),
                      mark_search(&search_calls, 'x'))) != sizeof(char *)
            || text_calls != 1 || search_calls != 1) {
        return 5;
    }
    if (!strchr(text, 'b') || strchr(text, 'z')) {
        return 6;
    }
    if (strchr(text, 'n') + 1 != &text[3]) {
        return 7;
    }

    return 0;
}
