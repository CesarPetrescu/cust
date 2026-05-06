struct Point {
    int x;
    char y;
};

struct Config {
    const int magic;
    int value;
    char marker;
};

struct Point global_point = {2, 3};
const struct Config global_config = {10, 4, 5};

int bump_static(void) {
    static struct Point saved = {7, 8};
    saved.x += 1;
    return saved.x + saved.y;
}

int sum_point(struct Point p) {
    return p.x + p.y;
}

int main(void) {
    struct Point local = {global_point.x + 1, 4};
    struct Point partial = {9, 0};
    const struct Config local_config = {6, 1, 0};

    int first = bump_static();
    int second = bump_static();

    return sum_point(local)
        + partial.x + partial.y
        + global_config.magic + global_config.value + global_config.marker
        + local_config.magic + local_config.value + local_config.marker
        + first + second;
}
