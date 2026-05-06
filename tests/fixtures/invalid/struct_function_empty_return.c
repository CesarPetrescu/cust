struct Point {
    int x;
};

struct Point bad() {
    return;
}

int main() {
    struct Point p;
    p = bad();
    return p.x;
}
