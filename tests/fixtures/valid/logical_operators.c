int main() {
    int x = 0;

    if (0 && (10 / x)) {
        return 99;
    } else {
        x = +5;
    }

    if (1 || (10 / 0)) {
        x = x + 1;
    }

    return (!0) + (!x) + (x && 2) + (0 || x) + (0 || 1 && 0 == 0);
}
