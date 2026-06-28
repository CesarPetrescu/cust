struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[2];
};

int main(void) {
    return ((struct Line){{{1, 2}, {3, 4}}}).points < ((struct Line){{{5, 6}, {7, 8}}}).points;
}
