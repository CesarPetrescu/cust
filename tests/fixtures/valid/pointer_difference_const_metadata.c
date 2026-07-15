int main(void) {
    int values[6] = {1, 2, 3, 4, 5, 6};
    const int *const_left = values + 1;
    const int *const_right = values + 4;
    int *cursor = values + 5;
    const int *const_base = values + 4;

    int *plus = values + (const_right - const_left);
    int *minus = cursor - (const_right - const_left);
    const int *kept_const = const_base - (const_right - const_left);

    return *plus + *minus + *kept_const;
}
