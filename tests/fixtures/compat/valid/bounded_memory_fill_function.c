void *memset(void *destination, int value, unsigned long int count);

int main(void) {
    char bytes[7] = {'x', 'x', 'x', 'x', 'x', 'x', 0};
    void *result = memset(bytes + 1, 'A', 3);
    if (result != bytes + 1 ||
        bytes[0] != 'x' || bytes[1] != 'A' || bytes[2] != 'A' ||
        bytes[3] != 'A' || bytes[4] != 'x' || bytes[5] != 'x' ||
        bytes[6] != 0) {
        return 1;
    }

    if (memset(bytes + 4, 0x123, 1) != bytes + 4 || bytes[4] != '#') {
        return 2;
    }

    unsigned char high_bytes[2] = {0, 0};
    memset(high_bytes, -1, 2);
    if (high_bytes[0] != 255 || high_bytes[1] != 255) {
        return 3;
    }

    if (memset(bytes + 5, 0, 0) != bytes + 5 || bytes[5] != 'x') {
        return 4;
    }

    return 0;
}
