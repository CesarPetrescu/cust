void *memcpy(void *destination, const void *source, unsigned long int count);
void *memmove(void *destination, const void *source, unsigned long int count);
int memcmp(const void *left, const void *right, unsigned long int count);
void *memset(void *destination, int value, unsigned long int count);
void *memchr(const void *source, int value, unsigned long int count);

int main(void) {
    int source[2] = {17, 29};
    int destination[3] = {41, 43, 47};

    if (memcpy(destination + 1, source, sizeof(source)) != destination + 1) return 1;
    if (destination[0] != 41 || destination[1] != 17 || destination[2] != 29) return 2;
    if (memcmp(destination + 1, source, sizeof(source)) != 0) return 3;

    if (memmove(destination + 1, destination, sizeof(destination[0]) * 2) !=
        destination + 1) return 4;
    if (destination[0] != 41 || destination[1] != 41 || destination[2] != 17) return 5;

    int cleared = 53;
    if (memset(&cleared, 0, sizeof(cleared)) != &cleared || cleared != 0) return 6;
    if (memchr(&cleared, 0, sizeof(cleared)) == 0) return 7;

    _Bool flag = 1;
    if (memset(&flag, 0, sizeof(flag)) != &flag || flag != 0) return 8;
    return 0;
}