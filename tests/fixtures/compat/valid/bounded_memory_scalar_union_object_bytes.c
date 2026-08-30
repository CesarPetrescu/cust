void *memcpy(void *, const void *, unsigned long int);
void *memmove(void *, const void *, unsigned long int);
int memcmp(const void *, const void *, unsigned long int);
void *memset(void *, int, unsigned long int);
void *memchr(const void *, int, unsigned long int);

union Scalar {
    int first;
    int second;
};

int main(void) {
    int source = 37;
    unsigned char zero_bytes[sizeof(int)] = {0};
    union Scalar value = {.first = 11};
    union Scalar copy = {.second = 19};

    if ((void *)&value.first != (void *)&value.second) return 1;
    if (memcpy(&value.second, &source, sizeof(source)) != (void *)&value.first) return 2;
    if (memcmp(&value.first, &source, sizeof(source)) != 0) return 3;
    if (memmove(&copy.first, &value.second, sizeof(value.second)) !=
        (void *)&copy.second) return 4;
    if (memcmp(&copy.second, &source, sizeof(source)) != 0) return 5;
    if (memset(&value.second, 0, sizeof(value.second)) != (void *)&value.first) return 6;
    if (memcmp(&value.first, zero_bytes, sizeof(zero_bytes)) != 0) return 7;
    if (memchr(&value.second, 0, sizeof(value.second)) != (void *)&value.first) return 8;
    return 0;
}
