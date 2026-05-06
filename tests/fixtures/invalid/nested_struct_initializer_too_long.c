struct Point {
    int x;
    char y;
};

struct Rect {
    struct Point origin;
    int width;
};

int main() {
    struct Rect rect = {{1, 2, 3}, 4};
    return rect.width;
}
