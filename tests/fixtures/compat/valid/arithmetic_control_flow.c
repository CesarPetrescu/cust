int main() {
    int total = 0;
    int i = 0;

    while (i < 6) {
        if (i == 3) {
            i = i + 1;
            continue;
        }

        total = total + (i * 2);
        if (total > 14) {
            break;
        }
        i = i + 1;
    }

    return total + (1 && 2) + (!0) + (5 % 3);
}
