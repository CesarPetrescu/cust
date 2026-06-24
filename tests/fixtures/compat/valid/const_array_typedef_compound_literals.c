struct Point { int x; int y; };

typedef const int Scores[2];
typedef const struct Point Points[2];

int sum_scores(const int *scores) {
    return scores[0] + scores[1];
}

int sum_points(const struct Point *points) {
    return points[0].x + points[1].y;
}

int main(void) {
    const int *scores = (Scores){3, 5};
    const struct Point *points = (Points){{7, 11}, {.x = 13, .y = 17}};
    return scores[0] + scores[1]
        + sum_scores((Scores){19, 23})
        + points[0].x + points[1].y
        + sum_points((Points){{29, 31}, {.x = 37, .y = 41}});
}
