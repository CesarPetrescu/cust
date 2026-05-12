int sum_values(int values[]) {
    return values[0] + values[1] + values[2] + values[3] + values[4] + values[5];
}

int main(void) {
    int values[] = {1, 2, [4] = 5, 6};
    char word[] = "cat";

    return sum_values(values) + word[3];
}
