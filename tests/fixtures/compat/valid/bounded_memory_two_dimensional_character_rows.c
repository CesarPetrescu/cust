void *memcpy(void *destination, const void *source, unsigned long int count);
void *memmove(void *destination, const void *source, unsigned long int count);
int memcmp(const void *left, const void *right, unsigned long int count);
void *memset(void *destination, int value, unsigned long int count);
void *memchr(const void *source, int value, unsigned long int count);

struct Rows {
    char values[2][5];
};

int copy_adjusted(char destination[][5], const char source[][5]) {
    if (memcpy(destination[1] + 1, source[0] + 1, 3) != destination[1] + 1) {
        return 1;
    }
    return memcmp(destination[1] + 1, source[0] + 1, 3) != 0;
}

int main(void) {
    char source[2][5] = {{'a', 'b', 'c', 'd', 0}, {'e', 'f', 'g', 'h', 0}};
    char destination[3][5] = {{0}, {'x', 'x', 'x', 'x', 0}, {0}};

    if (copy_adjusted(destination, source) != 0) return 1;
    if (destination[1][0] != 'x' || destination[1][1] != 'b' ||
        destination[1][2] != 'c' || destination[1][3] != 'd' ||
        destination[1][4] != 0) return 2;

    char (*row)[5] = destination + 1;
    if (memmove(*row + 2, *row + 1, 2) != *row + 2) return 3;
    if ((*row)[1] != 'b' || (*row)[2] != 'b' || (*row)[3] != 'c') return 4;
    if (memset(*row, '#', 1) != *row || (*row)[0] != '#') return 5;
    if (memchr(*row, 'c', 5) != *row + 3) return 6;

    struct Rows rows = {{{'m', 'n', 'o', 'p', 0}, {'q', 'r', 's', 't', 0}}};
    struct Rows *slot = &rows;
    if (memcpy(slot->values[1], rows.values[0], 4) != slot->values[1]) return 7;
    if (memcmp(slot->values[1], rows.values[0], 5) != 0) return 8;
    if (memchr(rows.values[1] + 1, 'p', 3) != rows.values[1] + 3) return 9;

    return 0;
}
