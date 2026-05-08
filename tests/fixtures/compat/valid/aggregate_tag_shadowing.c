typedef struct Point {
    int x;
} Point;

typedef union Number {
    int value;
    char tag;
} Number;

int use_outer(Point p) {
    return p.x;
}

int main(void) {
    Point outer = {4};
    Number first = {5};
    int total = use_outer(outer) + first.value;

    {
        typedef struct Point {
            int y;
            int z;
        } Point;
        typedef union Number {
            int value;
        } Number;

        Point inner = {6, 7};
        Number second = {8};
        total = total + inner.y + inner.z + second.value;
    }

    Point after = {9};
    Number final = {10};
    return total + use_outer(after) + final.value;
}
