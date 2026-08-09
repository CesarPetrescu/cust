void *memcpy(void *destination, const void *source, unsigned long int count);
void *memmove(void *destination, const void *source, unsigned long int count);
int memcmp(const void *left, const void *right, unsigned long int count);
void *memset(void *destination, int value, unsigned long int count);
void *memchr(const void *source, int value, unsigned long int count);

struct Matrix {
    int values[2][3];
    _Bool flags[2][3];
};

int copy_adjusted(int destination[][3], const int source[][3]) {
    if (memcpy(destination[1], source[0], sizeof(source[0][0]) * 3) != destination[1]) {
        return 1;
    }
    return memcmp(destination[1], source[0], sizeof(source[0][0]) * 3) != 0;
}

int main(void) {
    int source[2][3] = {{11, 13, 17}, {19, 23, 29}};
    int destination[3][3] = {{31, 37, 41}, {43, 47, 53}, {59, 61, 67}};

    if (copy_adjusted(destination, source) != 0) return 1;
    if (destination[1][0] != 11 || destination[1][1] != 13 || destination[1][2] != 17) return 2;

    int (*row)[3] = destination + 1;
    if (memmove(*row + 1, *row, sizeof((*row)[0]) * 2) != *row + 1) return 3;
    if ((*row)[0] != 11 || (*row)[1] != 11 || (*row)[2] != 13) return 4;
    if (memset(*row, 0, sizeof((*row)[0])) != *row || (*row)[0] != 0) return 5;
    if (memchr(*row, 0, sizeof((*row)[0])) != *row) return 6;

    struct Matrix matrix = {{{71, 73, 79}, {83, 89, 97}}, {{1, 1, 1}, {1, 1, 1}}};
    struct Matrix *slot = &matrix;
    if (memcpy(slot->values[1], source[1], sizeof(source[1][0]) * 3) != slot->values[1]) return 7;
    if (memcmp(matrix.values[1], source[1], sizeof(source[1][0]) * 3) != 0) return 8;
    if (memset(slot->flags[1], 0, sizeof(slot->flags[1][0]) * 3) != slot->flags[1]) return 9;
    if (matrix.flags[1][0] || matrix.flags[1][1] || matrix.flags[1][2]) return 10;

    return destination[0][0] != 31 || destination[2][2] != 67 || matrix.values[0][0] != 71;
}
