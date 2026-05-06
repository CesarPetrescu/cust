struct Point {
    int x;
    int y;
};

int main(void) {
    struct Point point = {1, 2, 3};
    return point.x;
}
