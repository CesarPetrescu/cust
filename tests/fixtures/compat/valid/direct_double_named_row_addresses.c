int main(void) {
    double values[2][2] = {{1.25, 2.5}, {3.75, 4.5}};
    double (*row)[2] = &values[1];
    const double (*view)[2] = &values[0];
    int marker = 0;
    if (sizeof(&values[marker++]) != sizeof(row) || marker != 0) return 1;
    if (sizeof(1 ? &values[0] : &values[0]) != sizeof(row)) return 4;
    if (row != values + 1 || &values[0] != values || row - &values[0] != 1) return 2;
    if (view[1][0] != 3.75) return 3;
    row[0][1] = 5.25;
    return (int)(row[0][0] * 4);
}
