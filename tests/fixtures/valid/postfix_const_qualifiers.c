struct Point {
    int x;
    int y;
};

typedef int const ConstInt;
typedef struct Point const ConstPoint;

int const read_int(int const value) {
    return value;
}

unsigned const int add_widened(unsigned const int left, unsigned const int right) {
    return left + right;
}

int read_point(struct Point const point) {
    return point.x + point.y;
}

int read_pointer(int const *values) {
    return values[0] + values[1];
}

int main(void) {
    int const local = 3;
    unsigned const int widened = 4;
    long const int longer = 5;
    ConstInt alias_value = 6;
    struct Point const point = {7, 8};
    ConstPoint alias_point = {9, 10};
    int values[2] = {11, 12};
    int const *view = values;

    return read_int(local) + add_widened(widened, 0) + longer + alias_value
        + read_point(point) + read_point(alias_point) + read_pointer(view);
}
