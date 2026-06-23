struct Box {
    const struct { int x; int y; } point;
};

int main(void) {
    struct Box box = {{1, 2}};
    box.point.y = 4;
    return box.point.y;
}
