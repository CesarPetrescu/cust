int marker = 0;

struct Point {
    int x;
    char tag;
};

union Number {
    int value;
    char tag;
};

struct Point make_point(int base) {
    marker = marker + 100;
    struct Point point = {base, 'a'};
    return point;
}

union Number make_number(int value) {
    marker = marker + 1000;
    union Number number = {value};
    return number;
}

int main(void) {
    struct Point left = {1, 'b'};
    struct Point right = {3, 'c'};
    union Number number = {5};
    int total = 0;

    total = total + sizeof((left = right).x);
    total = total + sizeof((1 ? left : right).tag);
    total = total + sizeof((marker = marker + 10, right).x);
    total = total + sizeof(make_point(7).tag);
    total = total + sizeof((number = make_number(8)).value);
    total = total + sizeof((1 ? number : make_number(9)).tag);

    return total + marker + left.x;
}
