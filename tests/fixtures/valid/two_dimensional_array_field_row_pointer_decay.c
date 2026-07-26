struct Frame {
    int matrix[3][3];
};

int calls = 0;

int (*select_rows(int (*rows)[3], int offset))[3] {
    calls += 1;
    return rows + offset;
}

int read_rows(int rows[][3]) {
    return rows[0][0] + rows[1][2];
}

int main(void) {
    struct Frame frames[2] = {
        {{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}},
        {{{10, 11, 12}, {13, 14, 15}, {16, 17, 18}}}
    };
    struct Frame *slot = &frames[1];
    int outer = 0;
    int marker = 0;
    int (*row)[3] = frames[outer++].matrix;

    select_rows(row, 1)[0][1] = 25;
    (marker ? row : slot->matrix + 1)[0][2] += 1;
    int before = (marker += 1, row + 2)[0][0]++;
    int updated = ++(slot->matrix + 2)[0][1];
    int literal = read_rows(((struct Frame){{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}}).matrix);

    if (outer != 1 || calls != 1 || marker != 1) return 1;
    if (frames[0].matrix[1][1] != 25 || frames[0].matrix[2][0] != 8) return 2;
    if (slot->matrix[1][2] != 16 || slot->matrix[2][1] != 18) return 3;
    if (before != 7 || updated != 18 || literal != 7) return 4;
    if (read_rows(frames[0].matrix) != 7) return 5;
    return 0;
}
