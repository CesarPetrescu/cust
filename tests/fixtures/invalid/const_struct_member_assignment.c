struct Config {
    const int magic;
    int value;
};

int main() {
    struct Config config;
    config.magic = 7;
    return config.magic;
}
