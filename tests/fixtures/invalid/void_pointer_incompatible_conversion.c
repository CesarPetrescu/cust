int main(void) {
    int value = 1;
    void *pointer = &value;
    char *wrong = pointer;
    return *wrong;
}
