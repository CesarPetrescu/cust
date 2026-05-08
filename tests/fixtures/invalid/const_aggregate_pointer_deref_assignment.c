struct Point {
    int x;
    int y;
};

int main(void) {
    const struct Point source = {1, 2};
    struct Point replacement = {3, 4};
    const struct Point *view = &source;
    *view = replacement;
    return source.x;
}
