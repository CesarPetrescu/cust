typedef const int ConstInt;
typedef const char ConstChar;

struct Point {
    int x;
    int y;
};

typedef const struct Point ConstPoint;
typedef const int ConstScores[3];

int read_const_int(ConstInt value) {
    return value + 1;
}

int sum_const_point(ConstPoint point) {
    return point.x + point.y;
}

int sum_const_scores(ConstScores);

int sum_const_scores(ConstScores scores) {
    static const int fallback[3] = {1, 1, 1};
    int original = scores[0] + scores[1] + scores[2];
    scores = fallback;
    return original + scores[0];
}

int main(void) {
    ConstInt value = 5;
    ConstChar marker = 2;
    ConstPoint point = {7, 8};
    ConstScores scores = {3, 4, 5};

    return read_const_int(value)
        + marker
        + sum_const_point(point)
        + scores[0]
        + scores[1]
        + scores[2]
        + sum_const_scores(scores);
}
