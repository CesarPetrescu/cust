int main(void) {
    int values[2] = {1, 2};
    void *left = values;
    void *right = values;
    return left - right;
}
