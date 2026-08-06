void *memchr(const void *source, int value, unsigned long int count);

int source_calls = 0;
int value_calls = 0;
int count_calls = 0;
char called_bytes[4] = {'x', 'y', 'z', 0};

const void *source_once(void) {
    source_calls += 1;
    return called_bytes;
}

int value_once(void) {
    value_calls += 1;
    return 'y';
}

int count_once(void) {
    count_calls += 1;
    return 3;
}

int main(void) {
    char bytes[6] = {'a', 0, 'b', 'a', 'c', 0};
    if (memchr(bytes, 0, 6) != bytes + 1) {
        return 1;
    }
    if (memchr(bytes, 'a', 6) != bytes) {
        return 2;
    }
    if (memchr(bytes + 2, 'a', 3) != bytes + 3) {
        return 3;
    }
    if (memchr(bytes, 'q', 6) != 0) {
        return 4;
    }
    if (memchr(bytes, 'a', 0) != 0) {
        return 5;
    }

    signed char high_byte[2] = {-1, 0};
    if (memchr(high_byte, 0x1ff, 1) != high_byte) {
        return 6;
    }

    if (memchr(source_once(), value_once(), count_once()) != called_bytes + 1 ||
        source_calls != 1 || value_calls != 1 || count_calls != 1) {
        return 7;
    }

    return 0;
}
