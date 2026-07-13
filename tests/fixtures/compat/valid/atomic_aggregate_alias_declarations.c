struct Point {
    int x;
};

typedef struct Point Point;
typedef const Point *View;
typedef _Atomic(Point) AtomicPoint;

struct Box {
    _Atomic(Point) value;
    _Atomic(View) view;
    AtomicPoint *slot;
};

int main(void) {
    Point origin = {5};
    AtomicPoint point;
    struct Box box;
    box.view = &origin;
    box.slot = &point;
    return (sizeof(box.value) == sizeof(Point)) + box.view->x
        + (sizeof(*box.slot) == sizeof(AtomicPoint));
}