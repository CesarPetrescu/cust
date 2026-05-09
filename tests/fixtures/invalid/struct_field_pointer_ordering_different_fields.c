struct Point {
    int x;
};

struct Line {
    struct Point left[2];
    struct Point right[2];
};

int main(void) {
    struct Line line = {{{1}, {2}}, {{3}, {4}}};
    struct Point *left = line.left;
    struct Point *right = line.right;
    return left < right;
}
