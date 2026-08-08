void *memcpy(void *destination, const void *source, unsigned long int count);
void *memmove(void *destination, const void *source, unsigned long int count);
int memcmp(const void *left, const void *right, unsigned long int count);
void *memset(void *destination, int value, unsigned long int count);
void *memchr(const void *source, int value, unsigned long int count);

struct Cell {
    int scalar;
    _Bool flag;
    int values[2];
};

struct Nested {
    struct Cell inner;
};

struct Row {
    int scalar;
};

struct Grid {
    struct Row rows[2];
};

struct AnonymousHolder {
    struct {
        int scalar;
        int values[2];
    } inner;
};

int main(void) {
    struct Cell source = {17, 1, {23, 29}};
    struct Cell destination = {31, 1, {37, 41}};
    struct Cell *destination_pointer = &destination;

    if (memcpy(&destination.scalar, &source.scalar, sizeof(source.scalar)) !=
        &destination.scalar) return 1;
    if (destination.scalar != source.scalar) return 2;
    if (memcmp(&destination.scalar, &source.scalar, sizeof(source.scalar)) != 0) return 3;

    if (memmove(destination_pointer->values + 1, destination_pointer->values,
                sizeof(destination_pointer->values[0])) != destination_pointer->values + 1) return 4;
    if (destination.values[1] != destination.values[0]) return 5;

    if (memset(&destination.flag, 0, sizeof(destination.flag)) != &destination.flag) return 6;
    if (destination.flag != 0) return 7;
    if (memchr(&destination.flag, 0, sizeof(destination.flag)) != &destination.flag) return 8;

    struct Nested nested = {{43, 1, {47, 53}}};
    if (memcpy(nested.inner.values, source.values, sizeof(source.values)) !=
        nested.inner.values) return 9;
    if (memcmp(nested.inner.values, source.values, sizeof(source.values)) != 0) return 10;

    struct AnonymousHolder anonymous = {{59, {61, 67}}};
    struct AnonymousHolder *anonymous_pointer = &anonymous;
    if (memset(&anonymous_pointer->inner.scalar, 0,
               sizeof(anonymous_pointer->inner.scalar)) != &anonymous_pointer->inner.scalar) return 11;
    if (anonymous.inner.scalar != 0) return 12;

    struct Grid grid = {{{71}, {73}}};
    if (memcpy(&grid.rows[0].scalar, &grid.rows[1].scalar,
               sizeof(grid.rows[0].scalar)) != &grid.rows[0].scalar) return 13;
    if (grid.rows[0].scalar != 73) return 14;

    return 0;
}
