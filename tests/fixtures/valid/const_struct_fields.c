struct Config {
    const int magic;
    const char marker;
    int value;
    char delta;
};

struct Config global_config;

int read_config(struct Config config) {
    config.value += 2;
    config.delta++;
    return config.magic + config.marker + config.value + config.delta;
}

int main() {
    struct Config local;
    local.value = 30;
    local.delta = 4;

    global_config.value = local.value + 5;
    global_config.delta = local.delta + 1;

    return read_config(global_config) + local.magic + local.marker + local.value + local.delta;
}
