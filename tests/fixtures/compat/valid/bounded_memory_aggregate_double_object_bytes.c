void *memcpy(void *destination, const void *source, unsigned long count);
void *memmove(void *destination, const void *source, unsigned long count);
void *memset(void *destination, int value, unsigned long count);
int memcmp(const void *left, const void *right, unsigned long count);
void *memchr(const void *memory, int value, unsigned long count);

struct Sample {
    double scalar;
    double values[3];
};

struct Outer {
    struct Sample inner;
};

int main(void) {
    struct Outer source = {{1.5, {2.5, 3.5, 4.5}}};
    struct Outer destination = {{0.0, {0.0, 0.0, 0.0}}};

    if (memcpy(&destination.inner.scalar, &source.inner.scalar,
               sizeof(destination.inner.scalar)) != &destination.inner.scalar) {
        return 1;
    }
    if (destination.inner.scalar != source.inner.scalar) {
        return 2;
    }

    if (memcpy(destination.inner.values, source.inner.values,
               sizeof(destination.inner.values)) != destination.inner.values) {
        return 3;
    }
    if (memcmp(destination.inner.values, source.inner.values,
               sizeof(source.inner.values)) != 0) {
        return 4;
    }

    if (memmove(destination.inner.values + 1, destination.inner.values,
                sizeof(destination.inner.values[0]) * 2) !=
        destination.inner.values + 1) {
        return 5;
    }
    if (destination.inner.values[0] != 2.5 ||
        destination.inner.values[1] != 2.5 ||
        destination.inner.values[2] != 3.5) {
        return 6;
    }

    if (memset(&destination.inner.scalar, 0,
               sizeof(destination.inner.scalar)) != &destination.inner.scalar) {
        return 7;
    }
    if (memchr(&destination.inner.scalar, 0,
               sizeof(destination.inner.scalar)) == 0) {
        return 8;
    }

    return 0;
}
