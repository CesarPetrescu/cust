struct Config {
    const int magic;
    int value;
};

int main() {
    struct Config a;
    struct Config b;
    a.value = 1;
    b = a;
    return b.value;
}
