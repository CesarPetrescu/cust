struct Point {
    int x;
    int y;
};

int main(void) {
    struct Point points[2] = {{1, 2}, {3, 4}};
    const struct Point *p = &points[0];
    p[1].x = 9;
    return points[1].x;
}
