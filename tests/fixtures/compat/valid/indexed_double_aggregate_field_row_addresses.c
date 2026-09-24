struct Table { double rows[2][2]; };

int main(void) {
    struct Table tables[2] = {
        {{{1.0, 2.0}, {3.0, 4.0}}},
        {{{5.0, 6.0}, {7.0, 8.0}}}
    };
    struct Table copy = tables[0];
    int owner_index = 0;
    int row_index = 1;
    double (*row)[2] = &tables[owner_index++].rows[row_index++];
    if (owner_index != 1 || row_index != 2 || row != tables[0].rows + 1) return 1;
    row[0][0] = 9.0;
    if (copy.rows[1][0] != 3.0 || tables[0].rows[1][0] != 9.0
        || tables[1].rows[1][0] != 7.0) return 2;
    if (sizeof(&tables[owner_index++].rows[row_index++]) != sizeof(row)
        || owner_index != 1 || row_index != 2) return 3;
    return 14;
}
