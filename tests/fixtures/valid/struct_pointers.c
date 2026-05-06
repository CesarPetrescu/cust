struct Point {
    int x;
    char y;
};

void set_point(struct Point *p, int x, char y);
int sum_point(struct Point *p);

void set_point(struct Point *p, int x, char y) {
    p->x = x;
    (*p).y = y;
}

int sum_point(struct Point *p) {
    return p->x + (*p).y;
}

int main() {
    struct Point point;
    struct Point *p = &point;
    p->x = 3;
    p->y = 0;
    (*p).x += 4;
    ++p->y;
    if (p != &point) {
        return 1;
    }
    set_point(p, 10, 'A');
    if (point.x != 10) {
        return 2;
    }
    if ((*p).y != 'A') {
        return 3;
    }
    if (sum_point(&point) != 75) {
        return 4;
    }
    return 0;
}
