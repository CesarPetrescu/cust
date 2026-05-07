struct Point {
    int x;
    int y;
};

int main(void) {
    struct Point points[2] = {[2] = {1, 2}};
    return points[0].x;
}
