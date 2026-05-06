void overwrite(int *slot) {
    *slot = 9;
}

int main() {
    int value = 7;
    const int *read_only = &value;
    overwrite(read_only);
    return value;
}
