struct Config {
    const int magic;
    int value;
};

int main(void) {
    return ((struct Config){1, 2}).magic = 3;
}
