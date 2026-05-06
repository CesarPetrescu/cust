int main() {
    int value = 7;
    int *mutable = &value;
    const int *read_only = &value;
    mutable = read_only;
    *mutable = 9;
    return value;
}
