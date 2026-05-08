typedef int Count;

typedef struct Point {
    int x;
    int y;
} Point;

typedef union Number {
    int value;
    char tag;
} Number;

const int scalar_value(void);
const Count alias_scalar(void);
const char char_value(void);
const Point make_point(int x, int y);
const struct Point make_direct_point(void);
const Number make_number(int value);
const union Number make_direct_number(void);

const int scalar_value(void) {
    return 5;
}

const Count alias_scalar(void) {
    return 7;
}

const char char_value(void) {
    return 'A';
}

const Point make_point(int x, int y) {
    Point p = {x, y};
    return p;
}

const struct Point make_direct_point(void) {
    struct Point p = {2, 3};
    return p;
}

const Number make_number(int value) {
    Number n = {value};
    return n;
}

const union Number make_direct_number(void) {
    union Number n = {11};
    return n;
}

int main(void) {
    Point p = make_point(13, 17);
    struct Point direct = make_direct_point();
    Number n = make_number(19);
    union Number direct_number = make_direct_number();

    return scalar_value()
        + alias_scalar()
        + char_value()
        + p.x
        + p.y
        + direct.x
        + direct.y
        + n.value
        + direct_number.value;
}
