typedef int Count, *CountPtr, Counts[3];
typedef const int ConstCount, *ConstCountView, ConstCounts[2];

struct Point { int x; int y; };
typedef struct Point Point, *PointPtr, Points[2];
typedef struct { int value; int extra; } Anon, *AnonPtr, Anons[2];

int sum_points(PointPtr point) {
    return point->x + point->y;
}

int main(void) {
    Count values[3] = {2, 4, 6};
    CountPtr cursor = values + 1;
    Counts more = {1, 3, 5};
    ConstCount locked = 7;
    ConstCountView view = values;
    ConstCounts fixed = {8, 9};

    Point point = {10, 11};
    PointPtr point_slot = &point;
    Points pair = {{1, 2}, {3, 4}};

    Anon anon = {12, 13};
    AnonPtr anon_slot = &anon;
    Anons list = {{14, 15}, {16, 17}};

    return *cursor + more[2] + locked + view[0] + fixed[1]
        + sum_points(point_slot) + pair[1].y + anon_slot->extra + list[1].value;
}
