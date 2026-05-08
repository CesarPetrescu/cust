struct Point { int x; int y; };
union Number { int value; char tag; };
typedef struct Point Point;
typedef union Number Number;

int sum(Point point) {
    return point.x + point.y;
}

Point choose(int flag) {
    return flag ? (Point){.x = 7, .y = 8} : (Point){3, 4};
}

int main(void) {
    Point direct = (Point){1, 2};
    Point designated = (struct Point){.y = 5, .x = 4};
    Point assigned;
    assigned = (Point){.x = direct.x + designated.x, .y = 6};
    Number number = (Number){.tag = 9};
    Point picked = choose(1);
    return sum((Point){10, 11})
        + assigned.x
        + picked.y
        + number.value
        + ((Number){12}).tag;
}
