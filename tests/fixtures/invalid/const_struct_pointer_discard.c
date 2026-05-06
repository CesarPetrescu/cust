struct Point {
    int x;
    int y;
};

int main() {
    const struct Point point;
    struct Point *mutable = &point;
    return mutable == 0;
}
