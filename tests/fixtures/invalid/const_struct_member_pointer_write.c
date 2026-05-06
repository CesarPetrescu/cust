struct Config {
    const int magic;
    int value;
};

int main() {
    struct Config config;
    struct Config *ptr = &config;
    ptr->magic = 7;
    return ptr->magic;
}
