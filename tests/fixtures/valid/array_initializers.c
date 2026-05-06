int global_values[5] = {1, 2, 3};
char global_chars[4] = {1, 0, 2};

int sum(int *values, int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += values[i];
        i++;
    }
    return total;
}

int sum_chars(char *values, int len) {
    int total = 0;
    int i = 0;
    while (i < len) {
        total += values[i];
        i++;
    }
    return total;
}

int bump_static() {
    static int seen[3] = {4, 5};
    seen[0]++;
    seen[2] += 2;
    return seen[0] + seen[1] + seen[2];
}

int main(void) {
    int values[4] = {global_values[0] + 4, 7};
    char letters[3] = {3};
    const int readonly[2] = {8, 9};

    return sum(global_values, 5)
        + sum_chars(global_chars, 4)
        + sum(values, 4)
        + letters[0]
        + readonly[0]
        + readonly[1]
        + bump_static()
        + bump_static();
}
