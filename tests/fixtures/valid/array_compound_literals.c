int sum(int values[]) {
    values[1] += 5;
    return values[0] + values[1] + values[2];
}

int first_char(char text[]) {
    text[1] = text[1] + 1;
    return text[0] + text[1] + text[2];
}

int main(void) {
    int *values = (int[]){1, 2, 3};
    int total = values[0] + values[1] + values[2];

    total += sum((int[3]){4, 5, 6});

    int *designated = (int[]){[2] = 7, [0] = 3, 4};
    total += designated[0] + designated[1] + designated[2];

    char *letters = (char[]){'a', 'b', 0};
    total += first_char(letters) - 196;

    return total;
}
