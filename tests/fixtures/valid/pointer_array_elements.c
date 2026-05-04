int bump(int *slot) {
    *slot = *slot + 7;
    return *slot;
}

int pair_sum(int *slot) {
    return slot[0] + slot[1];
}

int main() {
    int values[4];
    values[0] = 1;
    values[1] = 2;
    values[2] = 3;
    values[3] = 4;

    int *middle = &values[1];
    *(&values[1]) = *middle + 3;
    values[2] = bump(&values[2]);

    return values[0] + values[1] * 10 + values[2] * 5 + pair_sum(&values[1]);
}
