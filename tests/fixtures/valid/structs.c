struct Point {
    int x;
    char y;
};

struct Pair {
    int left;
    int right;
};

int main() {
    struct Point p;
    p.x = 10;
    p.y = 3;

    struct Pair pair;
    pair.left = p.x;
    pair.right = p.y + 4;

    {
        struct Point p;
        p.x = 2;
        p.y = 1;
        pair.left = pair.left + p.x + p.y;
    }

    return pair.left + pair.right;
}
