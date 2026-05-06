struct Point {
    int x;
};

struct Rect {
    struct Point origin;
};

int main() {
    struct Rect rect;
    return rect.origin.z;
}
