char *strncat(char *restrict destination, const char *restrict source,
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
    char destination[16] = "cat";
    char source[4] = "dog";
    char partial[4] = "a";
    char raw[3] = {'x', 'y', 'z'};
    char unchanged[4] = "hi";

    char *returned = strncat(
        mark_destination(&destination_calls, destination),
        mark_source(&source_calls, source),
        mark_count(&count_calls, 2));
    if (returned != destination || destination_calls != 1 || source_calls != 1
            || count_calls != 1 || destination[0] != 'c'
            || destination[2] != 't' || destination[3] != 'd'
            || destination[4] != 'o' || destination[5] != 0) {
        return 1;
    }

    if (strncat(partial, raw, 2) != partial || partial[0] != 'a'
            || partial[1] != 'x' || partial[2] != 'y' || partial[3] != 0) {
        return 2;
    }

    if (strncat(unchanged, source, 0) != unchanged
            || unchanged[0] != 'h' || unchanged[1] != 'i'
            || unchanged[2] != 0) {
        return 3;
    }

    destination_calls = 0;
    source_calls = 0;
    count_calls = 0;
    if (sizeof(strncat(mark_destination(&destination_calls, destination),
                       mark_source(&source_calls, source),
                       mark_count(&count_calls, 1))) != sizeof(char *)
            || sizeof(*strncat(mark_destination(&destination_calls, destination),
                               mark_source(&source_calls, source),
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
