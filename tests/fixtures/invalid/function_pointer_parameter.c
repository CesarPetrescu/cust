int apply(int (*callback)(int), int value) {
    return callback(value);
}

int main(void) {
    return 0;
}
