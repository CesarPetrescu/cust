struct Point { int x; int y; };

int helper(int seed) {
    auto int local = seed + 1;
    register int total = local * 2;
    auto char marker = 'A';
    auto struct Point point = {3, 4};
    int values[3] = {1, 2, 3};
    register int *cursor = values + 1;

    total += *cursor;
    for (auto int i = 0; i < 2; i++) {
        total += i;
    }
    for (register int j = 0; j < 2; j++) {
        total += j;
    }

    return total + point.x + point.y + marker - 'A';
}

int main(void) {
    return helper(5);
}
