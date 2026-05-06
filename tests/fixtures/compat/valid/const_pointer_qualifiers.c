int sum_const(const int *p) {
    return p[0] + *p;
}

void write_through(int * const p) {
    *p += 2;
    p[1] += 3;
}

int main() {
    int values[3];
    values[0] = 5;
    values[1] = 7;
    values[2] = 11;

    const int *read = values;
    int total = sum_const(read);
    read = &values[1];
    total += *read;

    int * const locked = &values[0];
    write_through(locked);

    const int * const both = &values[2];
    total += *both;

    return total + values[0] + values[1] + values[2];
}
