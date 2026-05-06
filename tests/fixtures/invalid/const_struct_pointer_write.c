struct Point {
    int x;
    int y;
};

void mutate(const struct Point *point) {
    point->x = 1;
}

int main() {
    struct Point point;
    mutate(&point);
    return point.x;
}
