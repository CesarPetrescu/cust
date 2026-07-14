struct Point {
    int x;
    int y;
};

int main(void) {
    struct Point points[2] = {{1, 2}, {3, 4}};
    struct Point *pointer = points;
    struct Point *outside = &pointer[2];
    return outside->x;
}
