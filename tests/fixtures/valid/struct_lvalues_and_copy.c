struct Point {
    int x;
    char y;
};

int main() {
    struct Point a;
    a.x = 3;
    a.y = 4;

    struct Point b;
    b = a;
    a.x = 100;
    a.y = 100;

    b.x += 2;
    ++b.y;
    int assigned = (b.x = b.x + b.y);
    int post = b.y++;

    return assigned + post + b.y;
}
