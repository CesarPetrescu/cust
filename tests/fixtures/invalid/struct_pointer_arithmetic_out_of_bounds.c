struct Point {
    int x;
};

int main(void) {
    struct Point points[2] = {{1}, {2}};
    struct Point *p = &points[1];
    p = p + 1;
    return p->x;
}
