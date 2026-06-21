typedef int *IntPtr;
typedef const int *ConstIntView;
typedef int * const ConstIntSlot;

struct Point {
    int x;
    int y;
};

typedef struct Point Point;
typedef Point *PointPtr;
typedef const Point *ConstPointView;

struct Cursor {
    IntPtr head, tail;
    ConstIntView view, limit;
    ConstIntSlot fixed;
    Point points[2], backups[1];
    PointPtr current, next;
    ConstPointView read_only, end;
};

int sum_points(Point *points) {
    return points[0].x + points[1].y;
}

int main(void) {
    int values[4] = {10, 20, 30, 40};
    struct Point points[2] = {{1, 2}, {3, 4}};
    struct Cursor cursor = {
        values,
        values + 3,
        values + 1,
        values + 2,
        values,
        {{5, 6}, {7, 8}},
        {{9, 10}},
        points,
        points + 1,
        points,
        points + 1
    };

    cursor.head += 1;
    cursor.current->x = 11;
    cursor.next->y = 12;

    return *cursor.head + *cursor.tail + *cursor.view + *cursor.limit + *cursor.fixed
        + cursor.points[0].x + cursor.points[1].y + cursor.backups[0].x
        + cursor.read_only->x + cursor.end->y + sum_points(cursor.current);
}
