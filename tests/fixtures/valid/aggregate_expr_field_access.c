struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[2];
};

struct Point make_point(int base) {
    struct Point point = {base, base + 1};
    return point;
}

int main(void) {
    struct Point replacement = {7, 8};
    struct Point left = {1, 2};
    struct Point right = {3, 4};
    int total = (((struct Line){{{1, 2}, {3, 4}}}).points[0] = replacement).x;
    total = total + (((struct Line){{{2, 3}, {4, 5}}}).points[1] = (struct Point){6, 7}).y;
    total = total + (left = right).x;
    total = total + (1 ? left : replacement).y;
    int marker = 0;
    total = total + (marker = marker + 1, right).x;
    total = total + marker;
    total = total + make_point(5).y;
    return total;
}
