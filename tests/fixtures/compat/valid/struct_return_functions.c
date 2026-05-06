struct Point {
    int x;
    char y;
};

struct Pair {
    int x;
    int y;
};

struct Point make_point(int x, char y);

struct Point make_point(int x, char y) {
    struct Point p;
    p.x = x;
    p.y = y;
    return p;
}

struct Point choose_point(int flag) {
    struct Point first;
    struct Point second;
    first.x = 2;
    first.y = 3;
    second.x = 5;
    second.y = 7;
    if (flag)
        return first;
    return second;
}

void mutate_copy(struct Point p) {
    p.x = 99;
    if (p.x == 123) {
        return;
    }
}

int main() {
    struct Point a;
    struct Point b;
    a = make_point(4, 6);
    b = choose_point(0);
    mutate_copy(a);
    return a.x + a.y + b.x + b.y;
}
