char *strrchr(const char *text, int search);

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
    char *last = strrchr(text, 'a');
    char *normalized = strrchr(text, 256 + 'n');
    char *terminator = strrchr(text, 0);

    if (last != &text[5] || normalized != &text[4]
            || terminator != &text[6] || strrchr(text, 'z') != 0) {
        return 1;
    }
    if (strrchr(text + 2, 'a') != &text[5]) {
        return 2;
    }
    *last = 'A';
    if (text[5] != 'A') {
        return 3;
    }
    if (strrchr(mark_text(&text_calls, text),
                mark_search(&search_calls, 'b')) != text
            || text_calls != 1 || search_calls != 1) {
        return 4;
    }
    if (sizeof(strrchr(mark_text(&text_calls, text),
                       mark_search(&search_calls, 'x'))) != sizeof(char *)
            || text_calls != 1 || search_calls != 1) {
        return 5;
    }
    if (!strrchr(text, 'b') || strrchr(text, 'z')) {
        return 6;
    }
    if (strrchr(text, 'n') + 1 != &text[5]) {
        return 7;
    }

    return 0;
}
