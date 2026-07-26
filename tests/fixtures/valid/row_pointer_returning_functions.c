int (*tail(int (*)[3]))[3];

int (*tail(int (*rows)[3]))[3] {
    return rows + 1;
}

const int (*view(const int (*rows)[3]))[3] {
    return rows;
}

int main(void) {
    int values[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
    int marker = 0;
    int (*row)[3] = tail(values);
    int direct = tail(values)[1][2];
    int conditional = (marker ? values : tail(values))[0][1];
    int comma = (marker += 1, tail(values))[0][0];
    const int (*read_only)[3] = view(values);

    row[0][1] += 10;
    if (direct != 9 || conditional != 5 || comma != 4 || marker != 1) return 1;
    if (values[1][1] != 15 || row[1][2] != 9) return 2;
    if (read_only[2][0] != 7 || sizeof(tail(values)) != sizeof(int *)) return 3;
    return 0;
}
