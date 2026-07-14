struct Point {
    int x;
};

const struct Point *pick(const struct Point *points) {
    return points;
}

int main(void) {
    const struct Point points[2] = {{1}, {2}};
    struct Point *mutable = &pick(points)[1];
    return mutable->x;
}
