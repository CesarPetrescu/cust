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

    if ((x *= 2) != 26) {
        return 7;
    }
    if ((x /= y) != 5) {
        return 8;
    }
    if ((x %= 3) != 2) {
        return 9;
    }

    if ((values[0] *= 2) != 28) {
        return 10;
    }
    if ((values[1] /= 3) != 1) {
        return 11;
    }
    if ((values[2] %= 5) != 2) {
        return 12;
    }

    p = &values[0];
    if ((*p %= 10) != 8) {
        return 13;
    }

    for (x = 0; x < 4; x += 1) {
        y += x;
    }

    return x + y + values[0] + values[1] + values[2];
}
