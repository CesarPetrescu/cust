typedef int Scores[3];
typedef char Word[4];
struct Point { int x; int y; };
typedef struct Point Points[2];

int sum_points(struct Point *points) {
    points[1].x += 5;
    return points[0].x + points[0].y + points[1].x + points[1].y;
}

int sum_ints(int values[3]) {
    values[2] += 4;
    return values[0] + values[1] + values[2];
}

int main(void) {
    int *scores = (Scores){1, 2, 3};
    int total = sum_ints((Scores){4, 5, 6});
    char *word = (Word){"cat"};
    struct Point *points = (Points){{7, 8}, {.x = 9, .y = 10}};
    return scores[0] + scores[1] + scores[2]
        + total
        + word[0] - 'a'
        + word[1] - 'a'
        + word[2] - 'a'
        + word[3]
        + sum_points(points)
        + points[1].x;
}
