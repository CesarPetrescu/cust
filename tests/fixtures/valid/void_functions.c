void add_to_total(int *total, int value) {
    *total += value;
    return;
}

void add_all(int *total, int values[3]) {
    int i = 0;
    while (i < 3) {
        add_to_total(total, values[i]);
        i++;
    }
}

void maybe_add(int *total, int enabled, int value) {
    if (!enabled)
        return;
    add_to_total(total, value);
}

int main() {
    int values[3];
    values[0] = 4;
    values[1] = 5;
    values[2] = 6;

    int total = 0;
    add_all(&total, values);
    maybe_add(&total, 0, 100);
    maybe_add(&total, 1, 7);
    return total;
}
