const int *view(const int *value) {
    return value;
}

int main() {
    const int locked = 5;
    int *mutable = view(&locked);
    return *mutable;
}
