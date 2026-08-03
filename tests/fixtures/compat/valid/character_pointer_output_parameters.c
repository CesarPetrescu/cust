static void set_end(char **out, char *value) {
    if (out != 0) {
        *out = value;
    }
}

static void forward_end(char **out, char *value) {
    set_end(out, value);
}

static int output_is_null(char **out) {
    return out == 0;
}

static int assignment_size_matches_pointer(char **out, char *value) {
    return sizeof(*out = value) == sizeof(char *);
}

int main(void) {
    char text[] = "123abc";
    char *end = 0;

    set_end(0, text);
    if (!output_is_null(0) || end != 0) {
        return 1;
    }
    set_end(&end, text + 3);
    if (*end != 'a') {
        return 2;
    }
    set_end(&end, 0);
    if (end != 0) {
        return 3;
    }
    if (!assignment_size_matches_pointer(&end, text + 2) || end != 0) {
        return 4;
    }
    forward_end(&end, text + 1);
    return end[0] == '2' ? 0 : 5;
}
