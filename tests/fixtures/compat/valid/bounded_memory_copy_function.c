void *memcpy(void * restrict destination,
             const void * restrict source,
             unsigned long int count);

int main(void) {
    char source[6] = {'A', 0, 'B', 'C', 'D', 0};
    char destination[7] = {'x', 'x', 'x', 'x', 'x', 'x', 0};
    void *result = memcpy(destination + 1, source + 1, 4);

    if (result != destination + 1) {
        return 1;
    }
    if (destination[0] != 'x' || destination[1] != 0 ||
        destination[2] != 'B' || destination[3] != 'C' ||
        destination[4] != 'D' || destination[5] != 'x' ||
        destination[6] != 0) {
        return 2;
    }

    signed char negative_source[2] = {-1, -2};
    signed char negative_destination[2] = {0, 0};
    memcpy(negative_destination, negative_source, 2);
    if (negative_destination[0] != negative_source[0] ||
        negative_destination[1] != negative_source[1]) {
        return 3;
    }
    return 0;
}