int choose(int *p, int *same, int *other) {
    int score = 0;
    if (p) {
        score = score + 1;
    }
    if (!0) {
        score = score + 2;
    }
    if (!p) {
        score = score + 1000;
    }
    if (p == same) {
        score = score + 4;
    }
    if (p != other) {
        score = score + 8;
    }
    if (p != 0) {
        score = score + 16;
    }
    if (0 == p) {
        score = score + 1000;
    }
    if (0 != p) {
        score = score + 32;
    }
    if (other == &other[0]) {
        score = score + 64;
    }
    return score;
}

int main() {
    int values[2];
    values[0] = 5;
    values[1] = 7;
    int *first = values;
    int *second = &values[1];
    return choose(first, &values[0], second);
}
