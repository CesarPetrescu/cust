struct Point {
    int x;
    char y;
};

struct Rect {
    struct Point origin;
    int width;
};

int sum_point(struct Point p) {
    return p.x + p.y;
}

int main(void) {
    struct Rect rect;
    rect.origin.x = 10;
    rect.origin.y = 3;
    rect.width = 7;

    struct Point copy;
    copy = rect.origin;
    copy.x += 2;

    return rect.origin.x + rect.origin.y + rect.width + sum_point(rect.origin) + copy.x;
}
