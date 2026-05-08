struct Point {
    int x;
    int y;
};

int main(void) {
    struct Point *points = (struct Point[1]){{1, 2}, {3, 4}};
    return points[0].x;
}
