struct Point {
    int x;
    int y;
};

struct Packet {
    int values[3];
};

struct Line {
    struct Point points[2];
};

int mutate_int(int *slot) {
    *slot = *slot + 5;
    return *slot;
}

int sum_point(struct Point *point) {
    point->x = point->x + 7;
    return point->x + point->y;
}

int main(void) {
    int total = ((struct Packet){{1, 2, 3}}).values[1];
    total = total + mutate_int(&((struct Packet){{4, 5, 6}}).values[2]);
    total = total + ((struct Packet){.values = {7, 8, 9}}).values[0];
    total = total + ((struct Line){{{1, 2}, {3, 4}}}).points[1].y;
    total = total + sum_point(&((struct Line){{{5, 6}, {7, 8}}}).points[0]);
    return total;
}
