int sum_const(const int *values, int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += values[i];
        i++;
    }
    return total;
}

int main() {
    int values[3];
    values[0] = 4;
    values[1] = 5;
    values[2] = 6;

    int *mutable = values;
    const int *read_only = mutable;
    const int *also_read_only = read_only;
    const int *advanced = read_only + 1;

    return sum_const(mutable, 3)
        + sum_const(read_only, 3)
        + *advanced
        + (also_read_only == read_only);
}
