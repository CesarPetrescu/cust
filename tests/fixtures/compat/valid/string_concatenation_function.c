char *strcat(char *restrict destination, const char *restrict source);

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
    char destination[16] = "cat";
    char source[4] = "dog";
    char offset[8] = {'x', 'x', 'a', 0, 'q', 'r', 's', 0};
    char empty[4] = "";
    char unchanged[4] = "hi";

    char *returned = strcat(
        mark_destination(&destination_calls, destination),
        mark_source(&source_calls, source));
    if (returned != destination || destination_calls != 1 || source_calls != 1
            || destination[0] != 'c' || destination[2] != 't'
            || destination[3] != 'd' || destination[5] != 'g'
            || destination[6] != 0) {
        return 1;
    }

    if (strcat(offset + 2, "b") != offset + 2
            || offset[1] != 'x' || offset[2] != 'a' || offset[3] != 'b'
            || offset[4] != 0 || offset[5] != 'r') {
        return 2;
    }

    if (strcat(empty, "ok") != empty || empty[0] != 'o'
            || empty[1] != 'k' || empty[2] != 0) {
        return 3;
    }

    if (strcat(unchanged, "") != unchanged || unchanged[0] != 'h'
            || unchanged[1] != 'i' || unchanged[2] != 0) {
        return 4;
    }

    destination_calls = 0;
    source_calls = 0;
    if (sizeof(strcat(mark_destination(&destination_calls, destination),
                      mark_source(&source_calls, source))) != sizeof(char *)
            || sizeof(*strcat(mark_destination(&destination_calls, destination),
                              mark_source(&source_calls, source))) != sizeof(char)
            || destination_calls != 0 || source_calls != 0) {
        return 5;
    }

    returned[1] = 'A';
    if (destination[1] != 'A') {
        return 6;
    }

    return 0;
}
