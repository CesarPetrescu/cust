struct Point {
    int x;
    int y;
};

int sum(struct Point p) {
    return p.x + p.y;
}

int main(void) {
    struct Point points[2] = {{1, 2}, {3, 4}};
    struct Point replacement = {5, 6};
    struct Point returned = (points[0] = replacement);

    replacement.x = 9;
    points[1] = (struct Point){7, 8};

    struct Point *cursor = points;
    cursor[0] = (struct Point){11, 12};

    return sum(returned) + points[0].x + points[0].y + points[1].x + points[1].y;
}
