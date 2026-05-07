struct Point {
    int x;
    int y;
};

int main() {
    struct Point points[1] = {{1, 2}, {3, 4}};
    return points[0].x;
}
