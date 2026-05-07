struct Point {
    int x;
};

struct Size {
    int value;
};

struct Size make_size(void) {
    struct Size s = {4};
    return s;
}

int main(void) {
    struct Point p = make_size();
    return p.x;
}
