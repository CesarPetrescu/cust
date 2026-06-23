struct Point {
    int x;
    int y;
};

struct Box {
    const struct Point point;
};

int main(void) {
    struct Box box = {{1, 2}};
    box.point.x = 3;
    return box.point.x;
}
