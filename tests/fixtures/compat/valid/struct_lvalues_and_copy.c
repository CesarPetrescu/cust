struct Point {
    int x;
    int y;
};

int main() {
    struct Point a;
    a.x = 1;
    a.y = 2;
    struct Point b;
    b = a;
    a.x = 100;
    b.x += 3;
    ++b.y;
    return (b.x = b.x + b.y);
}
