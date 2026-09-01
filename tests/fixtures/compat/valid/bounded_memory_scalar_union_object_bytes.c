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

union Carrierless {
    const int wide;
    char low;
    _Bool truth;
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
    union Carrierless carrierless_source = {.low = 7};
    union Carrierless carrierless_copy = {.low = 0};
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
    if ((void *)&carrierless_source.wide != (void *)&carrierless_source.low ||
        (void *)&carrierless_source.low != (void *)&carrierless_source.truth) return 22;
    if (memcpy(&carrierless_copy.low, &carrierless_source.low,
               sizeof(carrierless_source.low)) != (void *)&carrierless_copy.truth) return 23;
    if (memcmp(&carrierless_copy.low, &carrierless_source.low,
               sizeof(carrierless_copy.low)) != 0) return 24;
    if (memmove(&carrierless_copy.low, &carrierless_copy.low,
                sizeof(carrierless_copy.low)) != (void *)&carrierless_copy.wide) return 25;
    if (memset(&carrierless_copy.truth, 0, sizeof(carrierless_copy.truth)) !=
        (void *)&carrierless_copy.low) return 26;
    if (memchr(&carrierless_source.low, 7, sizeof(carrierless_source.low)) !=
        (void *)&carrierless_source.wide) return 27;
    if (carrierless_copy.low != 0 || carrierless_copy.truth != 0 ||
        sizeof(union Carrierless) < sizeof(int)) return 28;
    return 0;
}
