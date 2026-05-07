int *bad(const int *value) {
    return value;
}

int main() {
    const int locked = 3;
    int *value = bad(&locked);
    return *value;
}
