const struct { int x; int y; } point = {1, 2};

int main(void) {
    point.x = 3;
    return point.x;
}
