struct Config {
    const int magic;
    const char marker;
    int value;
    char delta;
};

struct Config global_config;

int main() {
    global_config.value = 37;
    global_config.delta = 5;
    return global_config.magic + global_config.marker + global_config.value + global_config.delta;
}
