int main() {
    int i = 0;
    int sum = 0;

    do {
        i = i + 1;
        if (i == 2) {
            continue;
        }
        if (i == 5) {
            break;
        }
        sum = sum + i;
    } while (i < 10);

    do {
        sum = sum + 10;
    } while (0);

    return sum;
}
