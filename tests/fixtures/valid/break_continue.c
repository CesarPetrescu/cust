int main() {
    int i = 0;
    int sum = 0;

    while (i < 10) {
        i = i + 1;
        if (i == 3) {
            continue;
        }
        if (i == 7) {
            break;
        }
        sum = sum + i;
    }

    for (int j = 0; j < 6; j = j + 1) {
        if (j == 1) {
            continue;
        }
        if (j == 4) {
            break;
        }
        sum = sum + (j * 10);
    }

    return sum;
}
