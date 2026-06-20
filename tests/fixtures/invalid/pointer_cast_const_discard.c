int main(void) {
    const int value = 4;
    const int *view = &value;
    int *mutable = (int *)view;
    return *mutable;
}
