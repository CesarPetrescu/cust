struct Point {
    int x;
};

struct Line {
    const struct Point points[1];
};

int main(void) {
    struct Point *points = ((struct Line){{{5}}}).points;
    return points[0].x;
}
