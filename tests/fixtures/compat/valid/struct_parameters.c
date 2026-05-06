struct Point {
    int x;
    int y;
};

int sum(struct Point p);
void mutate_copy(struct Point p, int *out);

int sum(struct Point p) {
    return p.x + p.y;
}

void mutate_copy(struct Point p, int *out) {
    p.x += 10;
    p.y = p.y + 20;
    *out = p.x + p.y;
}

int main() {
    struct Point a;
    a.x = 3;
    a.y = 4;

    if (sum(a) != 7) {
        return 1;
    }

    int copy_total = 0;
    mutate_copy(a, &copy_total);
    if (copy_total != 37) {
        return 2;
    }
    if (a.x != 3 || a.y != 4) {
        return 3;
    }

    struct Point b;
    b = a;
    b.x = 8;
    if (sum(b) != 12) {
        return 4;
    }

    return 0;
}
