struct Point {
    int x;
};

struct Pair {
    int x;
};

struct Point bad() {
    struct Pair p;
    p.x = 1;
    return p;
}

int main() {
    struct Point p;
    p = bad();
    return p.x;
}
