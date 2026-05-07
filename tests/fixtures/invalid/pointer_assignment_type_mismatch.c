struct Point {
    int x;
};

struct Size {
    int width;
};

int main(void) {
    struct Point point = {1};
    struct Size size = {2};
    struct Point *p = &point;
    p = &size;
    return p->x;
}
