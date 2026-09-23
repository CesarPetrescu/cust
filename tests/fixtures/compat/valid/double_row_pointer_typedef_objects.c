typedef double (*Row)[2];
typedef const double (*ConstRow)[2];

int main(void) {
    double values[2][2] = {{1.25, 2.5}, {3.75, 4.5}};
    Row row = values;
    ConstRow view = row;
    if (sizeof(*row) != 2 * sizeof(double)) return 1;
    if (sizeof(row) != sizeof(double *)) return 2;
    row += 1;
    row[0][1] = row[0][0] + 0.5;
    if (view[1][1] != 4.25) return 3;
    if (row - values != 1) return 4;
    return (int)(row[0][1] * 4) + (int)(row - values);
}
