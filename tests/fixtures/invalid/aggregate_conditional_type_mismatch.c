struct Point {
    int x;
    int y;
};

struct Size {
    int width;
    int height;
};

struct Point make_point(void) {
    struct Point p = {1, 2};
    return p;
}

struct Size make_size(void) {
    struct Size s = {3, 4};
    return s;
}

int main(void) {
    struct Point p = 0 ? make_point() : make_size();
    return p.x;
}
