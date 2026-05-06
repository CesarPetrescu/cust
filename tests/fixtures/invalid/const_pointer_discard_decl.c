int main() {
    int value = 7;
    const int *read_only = &value;
    int *mutable = read_only;
    *mutable = 9;
    return value;
}
