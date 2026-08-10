void *memcpy(void *destination, const void *source, unsigned long int count);
void *memmove(void *destination, const void *source, unsigned long int count);
int memcmp(const void *left, const void *right, unsigned long int count);
void *memset(void *destination, int value, unsigned long int count);
void *memchr(const void *source, int value, unsigned long int count);

struct Inner {
    int value;
    _Bool flag;
};

struct Record {
    int prefix;
    int values[2];
    struct Inner inner;
};

int main(void) {
    struct Record source = {11, {13, 17}, {19, 1}};
    struct Record destination = {23, {29, 31}, {37, 1}};
    struct Record moved = {41, {43, 47}, {53, 1}};

    if (memcpy(&destination, &source, sizeof(source)) != &destination) return 1;
    if (destination.prefix != 11 || destination.values[0] != 13 ||
        destination.values[1] != 17 || destination.inner.value != 19 ||
        destination.inner.flag != 1) return 2;
    if (memcmp(&destination, &source, sizeof(source)) != 0) return 3;

    if (memmove(&moved, &destination, sizeof(destination)) != &moved) return 4;
    if (memcmp(&moved, &source, sizeof(source)) != 0) return 5;

    if (memset(&destination, 0, sizeof(destination)) != &destination) return 6;
    if (destination.prefix != 0 || destination.values[0] != 0 ||
        destination.values[1] != 0 || destination.inner.value != 0 ||
        destination.inner.flag != 0) return 7;
    if (memchr(&destination, 0, sizeof(destination)) == 0) return 8;

    return 0;
}