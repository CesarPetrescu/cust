int add(int a, int b) {
    return a + b;
}

int square(int x) {
    return x * x;
}

int adjust(int value) {
    int local = value + 1;
    return local;
}

int main() {
    int seed = 3;
    return add(square(seed), adjust(4));
}
