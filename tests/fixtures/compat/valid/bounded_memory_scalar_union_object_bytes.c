void *memcpy(void *, const void *, unsigned long int);
void *memmove(void *, const void *, unsigned long int);
int memcmp(const void *, const void *, unsigned long int);
void *memset(void *, int, unsigned long int);
void *memchr(const void *, int, unsigned long int);

union Scalar {
    int first;
    int second;
};

union Flags {
    _Bool first;
    _Bool second;
};

int main(void) {
    int source = 37;
    unsigned char zero_bytes[sizeof(int)] = {0};
    unsigned char whole_zero_bytes[sizeof(union Scalar)] = {0};
    union Scalar value = {.first = 11};
    union Scalar copy = {.second = 19};
    union Scalar whole_source = {.first = 41};
    union Scalar whole_copy = {.second = 0};
    union Flags flags = {.first = 1};
    union Flags flags_copy = {.second = 0};
    unsigned char bool_zero_bytes[sizeof(union Flags)] = {0};

    if ((void *)&value.first != (void *)&value.second) return 1;
    if (memcpy(&value.second, &source, sizeof(source)) != (void *)&value.first) return 2;
    if (memcmp(&value.first, &source, sizeof(source)) != 0) return 3;
    if (memmove(&copy.first, &value.second, sizeof(value.second)) !=
        (void *)&copy.second) return 4;
    if (memcmp(&copy.second, &source, sizeof(source)) != 0) return 5;
    if (memset(&value.second, 0, sizeof(value.second)) != (void *)&value.first) return 6;
    if (memcmp(&value.first, zero_bytes, sizeof(zero_bytes)) != 0) return 7;
    if (memchr(&value.second, 0, sizeof(value.second)) != (void *)&value.first) return 8;

    if (memcpy(&whole_copy, &whole_source, sizeof(whole_source)) != &whole_copy) return 9;
    if (memcmp(&whole_copy, &whole_source, sizeof(whole_copy)) != 0) return 10;
    if (memmove(&whole_copy, &whole_copy, sizeof(whole_copy)) != &whole_copy) return 11;
    if (memset(&whole_copy, 0, sizeof(whole_copy)) != &whole_copy) return 12;
    if (memcmp(&whole_copy, whole_zero_bytes, sizeof(whole_copy)) != 0) return 13;
    if (memchr(&whole_copy, 0, sizeof(whole_copy)) != &whole_copy) return 14;
    if (memcpy(&flags_copy, &flags, sizeof(flags)) != &flags_copy) return 15;
    if (memcmp(&flags_copy, &flags, sizeof(flags)) != 0) return 16;
    if (memmove(&flags_copy, &flags_copy, sizeof(flags_copy)) != &flags_copy) return 17;
    if (memset(&flags_copy, 0, sizeof(flags_copy)) != &flags_copy) return 18;
    if (memcmp(&flags_copy, bool_zero_bytes, sizeof(flags_copy)) != 0) return 19;
    if (memchr(&flags_copy, 0, sizeof(flags_copy)) != &flags_copy) return 20;
    if (flags_copy.first != 0 || flags_copy.second != 0) return 21;
    return 0;
}
