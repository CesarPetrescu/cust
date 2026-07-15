int main(void) {
    int values[4] = {1, 2, 3, 4};
    const int *const_base = values + 3;
    int *left = values;
    int *right = values + 1;
    int *discarded = const_base - (right - left);

    return *discarded;
}
