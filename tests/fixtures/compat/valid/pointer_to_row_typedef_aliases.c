typedef int (*Row)[3], (*OtherRow)[3];
typedef const int (*ConstRow)[3];
typedef int (* const FixedRow)[3];

int values[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
static Row global_row = values + 1;

Row tail(Row rows) {
    return rows + 1;
}

int read(ConstRow rows) {
    return rows[0][0] + rows[1][2];
}

int main(void) {
    Row first = values, second = tail(values);
    OtherRow third = values + 2;
    FixedRow fixed = values;

    fixed[0][0] += 1;
    second[0][1] += 10;
    first = first + 1;

    if (global_row[0][1] != 15 || first != second) return 1;
    if (third[0][2] != 9 || read(fixed) != 8) return 2;
    if (sizeof(Row) != sizeof(int *) || sizeof(first) != sizeof(Row)) return 3;
    if (_Alignof(Row) != _Alignof(int *)) return 4;
    return 0;
}
