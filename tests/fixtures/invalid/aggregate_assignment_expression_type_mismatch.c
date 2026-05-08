struct Point {
    int x;
};

struct Size {
    int value;
};

int main(void) {
    struct Point p = {1};
    struct Size s = {2};
    struct Point copy = (p = s);
    return copy.x;
}
