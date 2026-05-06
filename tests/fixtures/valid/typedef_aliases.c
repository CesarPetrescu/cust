typedef int Count;
typedef char Byte;

struct Point {
    int x;
    char y;
};

typedef struct Point Point;

Count add(Count left, Byte right);
Point make_point(Count x, Byte y);

Count global_count = 2;
Byte global_byte = 3;
Point global_point;

Count add(Count left, Byte right) {
    Count total = left + right;
    return total;
}

Point make_point(Count x, Byte y) {
    Point p;
    p.x = x;
    p.y = y;
    return p;
}

int main() {
    Count values[3];
    values[0] = global_count;
    values[1] = global_byte;
    values[2] = add(values[0], values[1]);

    Count *slot = &values[2];
    *slot += 4;

    Point local;
    local = make_point(*slot, 'A');
    global_point = local;

    return sizeof(Count) + sizeof(Byte) + sizeof(Point) + global_point.x + (global_point.y == 'A');
}
