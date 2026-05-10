typedef int Scores[3];
typedef char Word[4];

struct Point { int x; int y; };
typedef struct Point Point;
typedef Point Points[2];

Scores global_scores = {1, 2, 3};
Word global_word = "cat";

int sum_scores(Scores values) {
    values[1] = values[1] + 5;
    return values[0] + values[1] + values[2];
}

int sum_points(Points points) {
    points[1].y = points[1].y + 4;
    return points[0].x + points[0].y + points[1].x + points[1].y;
}

int main(void) {
    Scores local = {[2] = 7, [0] = 4, 5};
    const Scores fixed = {6, 1, 2};
    Points points = {{1, 2}, {.x = 3, .y = 4}};
    int total = sum_scores(local) + sum_points(points);
    total = total + global_scores[2] + global_word[0] - 'a';
    total = total + sizeof(Scores) / sizeof(int);
    total = total + sizeof(Word);
    total = total + sizeof(Points) / sizeof(Point);
    total = total + fixed[0] - fixed[1] - fixed[2];
    return total;
}
