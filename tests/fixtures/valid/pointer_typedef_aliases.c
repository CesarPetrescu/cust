struct Point {
    int x;
    char y;
};

typedef int *IntPtr;
typedef char *CharPtr;
typedef struct Point *PointPtr;

void bump(IntPtr value) {
    *value += 2;
}

char first(CharPtr text) {
    return text[0];
}

void set_point(PointPtr point) {
    point->x = point->x + 3;
    (*point).y = 'A';
}

int main() {
    int value = 4;
    IntPtr value_ptr = &value;
    bump(value_ptr);

    char text[2];
    text[0] = 'B';
    text[1] = 0;
    CharPtr text_ptr = text;

    struct Point point;
    point.x = 5;
    PointPtr point_ptr = &point;
    set_point(point_ptr);

    if (sizeof(IntPtr) != 8) {
        return 1;
    }
    if (sizeof(CharPtr) != 8) {
        return 2;
    }
    if (sizeof(PointPtr) != 8) {
        return 3;
    }
    if (*value_ptr != 6) {
        return 4;
    }
    if (first(text_ptr) != 'B') {
        return 5;
    }
    if (point.x != 8 || point.y != 'A') {
        return 6;
    }
    return 0;
}
