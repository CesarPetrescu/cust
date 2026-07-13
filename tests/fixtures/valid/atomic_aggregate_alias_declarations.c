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

Point origin = {5};

_Atomic(View) current(void) {
    return &origin;
}

AtomicPoint make(void) {
    AtomicPoint point = {7};
    return point;
}

int main(void) {
    AtomicPoint point = make();
    struct Box box = {{1}, &origin, &point};
    return box.value.x + box.view->x + box.slot->x + current()->x - 5;
}