int main() {
    int value = 1;
    const int *p = &value;
    *p = 2;
    return value;
}
