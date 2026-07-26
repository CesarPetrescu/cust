int global_values[2][3] = {{1, 2, 3}, {4, 5, 6}};
static int (*global_row)[3] = global_values + 1;

int update(int (*row)[3]);

int update(int (*row)[3]) {
    row[1][1] += 5;
    return row[0][2] + row[1][1];
}

int read(const int (*row)[3]) {
    return row[0][0] + row[1][2];
}

int main(void) {
    static int (*static_row)[3] = global_values;
    int values[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    int (*row)[3] = values + 1;
    int (* const fixed)[3] = values;
    fixed[0][0] += 1;

    if (global_row[0][1] != 5 || static_row[0][1] != 2 || fixed[0][0] != 2 || sizeof(row) != sizeof(int *)) return 1;
    if (update(row) != 19) return 2;
    if (values[2][1] != 13) return 3;
    if (read(fixed + 1) != 13) return 4;
    if (row - fixed != 1) return 5;
    return 0;
}
