int sum_values(int values[]) {
    return values[0] + values[1] + values[2] + values[3] + values[4] + values[5];
}

int main(void) {
    int values[] = {1, 2, [4] = 5, 6};
    char word[] = "cat";
    const int table[] = {[1] = 3, [3] = 4};

    if (sizeof(values) != 48) {
        return 1;
    }
    if (sizeof(word) != 4) {
        return 2;
    }
    if (sizeof(table) != 32) {
        return 3;
    }

    return sum_values(values) + word[0] - word[1] + word[2] + table[0] + table[1] + table[2] + table[3];
}
