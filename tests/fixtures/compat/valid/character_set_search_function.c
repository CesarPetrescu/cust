char *strpbrk(const char *text, const char *accept);

char *mark_text(int *calls, char *text) {
    *calls += 1;
    return text;
}

char *mark_accept(int *calls, char *accept) {
    *calls += 1;
    return accept;
}

int main(void) {
    int text_calls = 0;
    int accept_calls = 0;
    char text[12] = "abracadabra";
    char accept[4] = "xyc";
    char *first = strpbrk(text, accept);

    if (first != &text[4] || strpbrk(text, "rb") != &text[1]
            || strpbrk(text, "qz") != 0 || strpbrk(text, "") != 0) {
        return 1;
    }
    if (strpbrk(text + 3, "d") != &text[6]) {
        return 2;
    }
    if (strpbrk(mark_text(&text_calls, text),
                mark_accept(&accept_calls, accept)) != &text[4]
            || text_calls != 1 || accept_calls != 1) {
        return 3;
    }
    *first = 'C';
    if (text[4] != 'C') {
        return 4;
    }
    if (sizeof(strpbrk(mark_text(&text_calls, text),
                       mark_accept(&accept_calls, accept))) != sizeof(char *)
            || text_calls != 1 || accept_calls != 1) {
        return 5;
    }
    if (!strpbrk(text, "C") || strpbrk(text, "z")) {
        return 6;
    }
    if (strpbrk(text, "d") + 1 != &text[7]) {
        return 7;
    }

    return 0;
}
