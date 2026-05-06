struct Point {
    int x;
};

struct Pair {
    int x;
};

int read_x(struct Point p) {
    return p.x;
}

int main() {
    struct Pair pair;
    return read_x(pair);
}
