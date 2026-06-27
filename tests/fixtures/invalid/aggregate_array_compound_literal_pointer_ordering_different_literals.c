struct Point {
    int x;
    int y;
};

int main(void) {
    return ((struct Point[]){{1, 2}, {3, 4}}) < ((struct Point[]){{5, 6}, {7, 8}});
}
