int main() {
    int value = 4;
    int *p = &value;
    return *(p + 1);
}
