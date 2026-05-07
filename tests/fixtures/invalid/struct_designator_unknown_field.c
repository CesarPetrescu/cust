struct Point {
    int x;
    int y;
};

int main(void) {
    struct Point p = {.z = 1};
    return p.x;
}
