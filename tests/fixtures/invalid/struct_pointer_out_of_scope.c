struct Point {
    int x;
};

struct Point *saved;

void capture() {
    struct Point point;
    point.x = 7;
    saved = &point;
}

int main() {
    capture();
    return saved->x;
}
