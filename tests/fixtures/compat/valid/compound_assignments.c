int main() {
    int x = 10;
    int y = 3;
    int values[3];
    int *p = &x;

    values[0] = 4;
    values[1] = 8;
    values[2] = 0;

    if ((x += 5) != 15) {
        return 1;
    }
    if ((x -= y += 2) != 10) {
        return 2;
    }

    if ((values[0] += x) != 14) {
        return 3;
    }
    if ((values[1] -= values[0] - 9) != 3) {
        return 4;
    }

    if ((*p += values[1]) != 13) {
        return 5;
    }
    if ((*(&values[2]) += *p - 6) != 7) {
        return 6;
    }

    for (x = 0; x < 4; x += 1) {
        y += x;
    }

    return x + y + values[0] + values[1] + values[2];
}
