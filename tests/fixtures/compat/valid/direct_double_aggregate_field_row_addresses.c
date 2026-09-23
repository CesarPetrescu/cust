struct Table { double rows[2][2]; };

int main(void) {
    struct Table table = {{{1.5, 2.5}, {3.5, 4.5}}};
    struct Table copy = table;
    int marker = 1;
    double (*row)[2] = &copy.rows[marker++];
    if (marker != 2 || row != copy.rows + 1 || row - copy.rows != 1) return 1;
    if (sizeof(&copy.rows[marker++]) != sizeof(row) || marker != 2) return 2;
    row[0][1] = 6.5;
    if (table.rows[1][1] != 4.5 || copy.rows[1][1] != 6.5) return 3;
    return (int)(row[0][0] * 4);
}
