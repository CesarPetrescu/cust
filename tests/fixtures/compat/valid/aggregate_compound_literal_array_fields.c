struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[2];
};

int sum(struct Point *points) {
    points[1].x = points[1].x + 5;
    return points[0].x + points[1].x + points[1].y;
}

int main(void) {
    struct Point *points = ((struct Line){{{1, 2}, {3, 4}}}).points;
    int total = sum(points);
    total = total + (((struct Line){.points = {{5, 6}, {7, 8}}}).points + 1)->y;
    return total;
}
