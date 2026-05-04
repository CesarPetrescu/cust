int main() {
    int sum = 0;
    for (int i = 0; i < 5; i = i + 1) {
        sum = sum + i;
    }

    int product = 1;
    int j = 1;
    for (; j <= 4; j = j + 1) {
        product = product * j;
    }

    int skipped = 7;
    for (int k = 0; k < 0; k = k + 1) {
        skipped = skipped + 100;
    }

    return sum + product + skipped;
}
