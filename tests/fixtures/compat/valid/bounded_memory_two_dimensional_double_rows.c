void *memcpy(void *destination, const void *source, unsigned long int count);
void *memmove(void *destination, const void *source, unsigned long int count);
int memcmp(const void *left, const void *right, unsigned long int count);
void *memset(void *destination, int value, unsigned long int count);
void *memchr(const void *source, int value, unsigned long int count);

struct Matrix {
    double values[2][3];
};

typedef double MatrixRows[3][3];

static int typedef_row_distance(MatrixRows matrix) {
    return (matrix + 2) - matrix;
}

int copy_adjusted(double destination[][3], const double source[][3]) {
    if (memcpy(destination[1], source[0], sizeof(source[0][0]) * 3) != destination[1]) {
        return 1;
    }
    return memcmp(destination[1], source[0], sizeof(source[0][0]) * 3) != 0;
}

static int advance_const_view(const double source[][3]) {
    const double (*origin)[3] = source;
    source += 1;
    return (int)(source - origin);
}

int main(void) {
    double source[2][3] = {{1.5, 2.5, 3.5}, {4.5, 5.5, 6.5}};
    double destination[3][3] = {{7.5, 8.5, 9.5}, {10.5, 11.5, 12.5}, {13.5, 14.5, 15.5}};

    if (copy_adjusted(destination, source) != 0) return 1;
    if (destination[1][0] != 1.5 || destination[1][1] != 2.5 || destination[1][2] != 3.5) return 2;
    if (typedef_row_distance(destination) != 2) return 10;
    if (advance_const_view(source) != 1) return 11;

    double (*row)[3] = destination + 1;
    if (memmove(*row + 1, *row, sizeof((*row)[0]) * 2) != *row + 1) return 3;
    if ((*row)[0] != 1.5 || (*row)[1] != 1.5 || (*row)[2] != 2.5) return 4;
    unsigned char zero_bytes[sizeof(double)] = {0};
    if (memset(*row, 0, sizeof((*row)[0])) != *row) return 5;
    if (memcmp(*row, zero_bytes, sizeof((*row)[0])) != 0) return 5;
    if (memchr(*row, 0, sizeof((*row)[0])) != *row) return 6;

    struct Matrix matrix = {{{16.5, 17.5, 18.5}, {19.5, 20.5, 21.5}}};
    struct Matrix *slot = &matrix;
    if (memcpy(slot->values[1], source[1], sizeof(source[1][0]) * 3) != slot->values[1]) return 7;
    if (memcmp(matrix.values[1], source[1], sizeof(source[1][0]) * 3) != 0) return 8;
    if (matrix.values[1][0] != 4.5 || matrix.values[1][1] != 5.5 || matrix.values[1][2] != 6.5) return 9;

    return destination[0][0] != 7.5 || destination[2][2] != 15.5 || matrix.values[0][0] != 16.5;
}
