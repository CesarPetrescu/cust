typedef int (Count);
typedef int *(IntPtr), (Scores)[3];

struct Point {
    int x;
    int y;
};

typedef struct Point (PointAlias), *(PointPtr), (PointArray)[2];

int sum_scores(Scores scores) {
    return scores[0] + scores[1] + scores[2];
}

int move_point(PointPtr point, Count dx) {
    point->x += dx;
    return point->x + point->y;
}

int main(void) {
    Count value = 5;
    int raw[3] = {1, 2, 3};
    Scores scores = {4, 5, 6};
    IntPtr cursor = raw + 1;
    PointAlias point = {7, 8};
    PointArray points = {{1, 2}, {3, 4}};
    PointPtr point_ptr = &point;

    return value + *cursor + sum_scores(scores) + move_point(point_ptr, 2) + points[1].y;
}
