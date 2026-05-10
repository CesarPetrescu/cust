struct Point {
    int x;
    int y;
};

struct Box {
    struct Point inner;
    int tail;
};

int bump(int *slot, int delta) {
    *slot = *slot + delta;
    return *slot;
}

int main(void) {
    struct Point point = {4, 8};
    struct Point *point_ptr = &point;
    int *x = &point_ptr->x;
    int total = bump(x, 3);

    struct Box box = {{2, 5}, 9};
    struct Box *box_ptr = &box;
    int *nested = &box_ptr->inner.y;
    total = total + bump(nested, 4);

    struct Point *inner = &box_ptr->inner;
    int *through_inner = &inner->x;
    total = total + bump(through_inner, 6);

    return total + point.x + box.inner.x + box.inner.y;
}
