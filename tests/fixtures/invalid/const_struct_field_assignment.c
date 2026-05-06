struct Point {
    int x;
    int y;
};

int main() {
    const struct Point point;
    point.x = 1;
    return point.x;
}
