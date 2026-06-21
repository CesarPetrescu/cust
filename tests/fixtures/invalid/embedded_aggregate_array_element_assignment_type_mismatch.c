struct Point {
    int x;
};

struct Size {
    int width;
};

struct Line {
    struct Point points[1];
};

int main(void) {
    struct Line line = {{{1}}};
    struct Size size = {2};
    line.points[0] = size;
    return 0;
}
