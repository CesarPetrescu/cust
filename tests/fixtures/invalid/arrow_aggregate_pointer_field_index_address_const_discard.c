struct Point {
    int x;
};

struct Cursor {
    const struct Point *points;
};

int main(void) {
    struct Point points[2] = {{1}, {2}};
    struct Cursor cursor = {points};
    struct Cursor *view = &cursor;
    struct Point *mutable = &view->points[1];
    return mutable->x;
}
