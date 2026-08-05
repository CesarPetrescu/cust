int main(void) {
    const int value = 1;
    const void *pointer = &value;
    int *discarded = pointer;
    return *discarded;
}
