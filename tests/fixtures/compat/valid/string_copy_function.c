char *strcpy(char *restrict destination, const char *restrict source);

char *mark_destination(int *calls, char *destination) {
    *calls += 1;
    return destination;
}

const char *mark_source(int *calls, const char *source) {
    *calls += 1;
    return source;
}

int main(void) {
    int destination_calls = 0;
    int source_calls = 0;
    char destination[16] = "old contents";
    char source[6] = "hello";
    char offset[8] = {'x', 'x', 'x', 'x', 'x', 'x', 'x', 0};
    char first[8] = "first";
    char second[8] = "second";

    char *returned = strcpy(
        mark_destination(&destination_calls, destination),
        mark_source(&source_calls, source));
    if (returned != destination || destination_calls != 1 || source_calls != 1
            || destination[0] != 'h' || destination[4] != 'o'
            || destination[5] != 0 || destination[6] != 'n') {
        return 1;
    }

    if (strcpy(offset + 2, "cat") != offset + 2
            || offset[1] != 'x' || offset[2] != 'c' || offset[4] != 't'
            || offset[5] != 0 || offset[6] != 'x') {
        return 2;
    }

    if (strcpy(second, strcpy(first, "ok")) != second
            || first[0] != 'o' || first[2] != 0
            || second[0] != 'o' || second[2] != 0) {
        return 3;
    }

    destination_calls = 0;
    source_calls = 0;
    if (sizeof(strcpy(mark_destination(&destination_calls, destination),
                      mark_source(&source_calls, source))) != sizeof(char *)
            || sizeof(*strcpy(mark_destination(&destination_calls, destination),
                              mark_source(&source_calls, source))) != sizeof(char)
            || destination_calls != 0 || source_calls != 0) {
        return 4;
    }

    returned[1] = 'A';
    if (destination[1] != 'A') {
        return 5;
    }

    return 0;
}
