struct Point {
    int x;
};

struct Cursor {
    const struct Point *points;
};

int main(void) {
    struct Point points[2] = {{1}, {2}};
    struct Cursor cursor = {points};
    struct Point *mutable = &cursor.points[1];
    return mutable->x;
}
