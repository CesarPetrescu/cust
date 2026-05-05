int score(int x) {
    if (x < 0)
        return 3;
    else if (x == 0)
        return 5;
    else if (x == 1)
        return 7;
    else
        return 11;
}

int dangling_else(int x) {
    if (x) {
        if (0)
            return 20;
        else
            return 30;
    }
    return 40;
}

int main() {
    int total = 0;
    int i = 0;

    if (1)
        total += score(-1);
    else
        total += 100;

    total += score(0);
    total += score(1);
    total += score(2);
    total += dangling_else(1);
    total += dangling_else(0);

    while (i < 3)
        total += i++;

    for (int j = 0; j < 4; j++)
        total += j;

    i = 0;
    do
        total += ++i;
    while (i < 2);

    for (int k = 0; k < 5; k++)
        if (k == 2)
            continue;
        else
            total += k;

    while (1)
        break;

    return total;
}
