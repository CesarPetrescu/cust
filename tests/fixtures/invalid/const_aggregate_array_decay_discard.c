struct Point {
    int x;
};

void mutate(struct Point *points) {
    points[0].x = 9;
}

int main() {
    const struct Point points[1] = {{1}};
    mutate(points);
    return points[0].x;
}
