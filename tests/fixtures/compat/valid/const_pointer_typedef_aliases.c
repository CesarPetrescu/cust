typedef const int *ConstIntView;
typedef int * const ConstIntSlot;

struct Point {
    int x;
    int y;
};

typedef const struct Point *ConstPointView;
typedef struct Point * const PointSlot;

int read_int(ConstIntView values) {
    return values[1];
}

ConstIntView choose_ints(ConstIntView values) {
    return values + 2;
}

int read_point(ConstPointView point) {
    return point->x + point->y;
}

int bump_slot(ConstIntSlot values) {
    values[0] = values[0] + 5;
    return values[0];
}

int move_point_slot(PointSlot point) {
    point->x = point->x + 3;
    return point->x;
}

int main(void) {
    int values[3] = {4, 6, 8};
    struct Point point = {3, 9};

    ConstIntView view = values;
    ConstIntView tail = choose_ints(values);
    ConstIntSlot slot = values;
    ConstPointView point_view = &point;
    PointSlot point_slot = &point;

    return read_int(view) + tail[0] + bump_slot(slot) + read_point(point_view) + move_point_slot(point_slot) + values[0] + point.x;
}
