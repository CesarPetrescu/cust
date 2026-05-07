struct Point {
    const int x;
    int y;
};

int main(void) {
    struct Point p = {1, 2};
    int *px = &p.x;
    *px = 3;
    return p.y;
}
