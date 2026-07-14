struct Point {
    int x;
};

struct Cursor {
    struct Point *points;
};

int main(void) {
    struct Point points[2] = {{1}, {2}};
    struct Cursor cursor = {points};
    struct Cursor *view = &cursor;
    struct Point *outside = &view->points[2];
    return outside->x;
}
