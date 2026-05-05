int main() {
    int x = 0;
    int y = 0;
    int values[3];
    int *p = &x;

    if ((x = 3) != 3) {
        return 1;
    }

    y = (x = 4) + 5;
    values[0] = (values[1] = 7);

    if ((*p = 6) != 6) {
        return 2;
    }

    x = y = 5;
    if (x != 5 || y != 5) {
        return 3;
    }

    for (x = 0; (x = x + 1) < 4; y = y + x) {
        ;
    }

    return x + y + values[0] + values[1];
}
