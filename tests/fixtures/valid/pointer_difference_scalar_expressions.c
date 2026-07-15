int main(void) {
    int values[6] = {1, 2, 3, 4, 5, 6};
    int *left = values + 1;
    int *right = values + 4;
    int marker = 0;

    int direct = 5 + (right - left);
    int scaled = (right - left) * 4;
    int selected = 6 + (1 ? right - left : 2);
    int comma = 4 + (marker += 1, right - left);
    int truth = (right - left) ? 5 : 0;

    return direct + scaled + selected + comma + marker + truth;
}
