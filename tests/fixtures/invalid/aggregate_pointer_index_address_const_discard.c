struct Point {
    int x;
    int y;
};

int main(void) {
    const struct Point points[2] = {{1, 2}, {3, 4}};
    const struct Point *view = points;
    struct Point *mutable = &view[1];
    return mutable->x;
}
