struct Point {
    int x;
};

int read_x(struct Point p) {
    return p.x;
}

int main() {
    int value = 1;
    return read_x(value);
}
