char *strncpy(char *restrict destination, const char *restrict source,
              unsigned long int count);

char *mark_destination(int *calls, char *destination) {
    *calls += 1;
    return destination;
}

const char *mark_source(int *calls, const char *source) {
    *calls += 1;
    return source;
}

unsigned long int mark_count(int *calls, unsigned long int count) {
    *calls += 1;
    return count;
}

int main(void) {
    int destination_calls = 0;
    int source_calls = 0;
    int count_calls = 0;
    char destination[6] = {'x', 'x', 'x', 'x', 'x', 'x'};
    char truncated[4] = {'a', 'a', 'a', 'z'};
    char raw[3] = {'x', 'y', 'z'};
    char padded[4] = {'a', 'a', 'a', 'z'};

    char *returned = strncpy(
        mark_destination(&destination_calls, destination),
        mark_source(&source_calls, "cat"),
        mark_count(&count_calls, 5));
    if (returned != destination || destination_calls != 1 || source_calls != 1
            || count_calls != 1 || destination[0] != 'c'
            || destination[1] != 'a' || destination[2] != 't'
            || destination[3] != 0 || destination[4] != 0
            || destination[5] != 'x') {
        return 1;
    }

    if (strncpy(truncated, raw, 2) != truncated || truncated[0] != 'x'
            || truncated[1] != 'y' || truncated[2] != 'a'
            || truncated[3] != 'z') {
        return 2;
    }

    if (strncpy(padded, "q", 3) != padded || padded[0] != 'q'
            || padded[1] != 0 || padded[2] != 0 || padded[3] != 'z') {
        return 3;
    }

    destination_calls = 0;
    source_calls = 0;
    count_calls = 0;
    if (sizeof(strncpy(mark_destination(&destination_calls, destination),
                       mark_source(&source_calls, "x"),
                       mark_count(&count_calls, 1))) != sizeof(char *)
            || sizeof(*strncpy(mark_destination(&destination_calls, destination),
                               mark_source(&source_calls, "x"),
                               mark_count(&count_calls, 1))) != sizeof(char)
            || destination_calls != 0 || source_calls != 0 || count_calls != 0) {
        return 4;
    }

    returned[1] = 'A';
    if (destination[1] != 'A') {
        return 5;
    }

    return 0;
}
