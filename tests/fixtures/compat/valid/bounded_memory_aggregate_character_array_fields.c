void *memcpy(void *destination, const void *source, unsigned long int count);
void *memmove(void *destination, const void *source, unsigned long int count);
int memcmp(const void *left, const void *right, unsigned long int count);
void *memset(void *destination, int value, unsigned long int count);
void *memchr(const void *source, int value, unsigned long int count);

struct Buffers {
    char source[6];
    char destination[6];
};

struct Envelope {
    struct Buffers nested;
};

int main(void) {
    struct Envelope envelope = {
        {{'a', 'b', 'c', 'd', 0, 0}, {'x', 'x', 'x', 'x', 'x', 0}}
    };
    struct Envelope *slot = &envelope;

    if (memcpy(slot->nested.destination + 1,
               envelope.nested.source + 1,
               3) != slot->nested.destination + 1) {
        return 1;
    }
    if (memcmp(envelope.nested.destination + 1,
               envelope.nested.source + 1,
               3) != 0) {
        return 2;
    }
    if (memmove(slot->nested.destination + 2,
                slot->nested.destination + 1,
                3) != slot->nested.destination + 2) {
        return 3;
    }
    if (slot->nested.destination[1] != 'b' ||
        slot->nested.destination[2] != 'b' ||
        slot->nested.destination[3] != 'c' ||
        slot->nested.destination[4] != 'd') {
        return 4;
    }
    if (memset(envelope.nested.destination + 1, '#', 2) !=
        envelope.nested.destination + 1) {
        return 5;
    }
    if (memchr(slot->nested.destination, '#', 6) !=
        slot->nested.destination + 1) {
        return 6;
    }

    struct { char bytes[4]; } anonymous = {{'m', 'n', 'o', 0}};
    if (memchr(anonymous.bytes + 1, 'o', 2) != anonymous.bytes + 2) {
        return 7;
    }
    if (memcpy(envelope.nested.destination,
               anonymous.bytes,
               3) != envelope.nested.destination) {
        return 8;
    }
    if (memcmp(envelope.nested.destination, anonymous.bytes, 3) != 0) {
        return 9;
    }

    return 0;
}
