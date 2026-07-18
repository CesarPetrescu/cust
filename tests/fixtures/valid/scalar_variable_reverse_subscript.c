struct Point {
    int x;
    int y;
};

struct Index {
    int value;
};

int main(void) {
    int values[4] = {3, 5, 7, 11};
    const int fixed[3] = {13, 17, 19};
    struct Point points[3] = {{2, 4}, {6, 8}, {10, 12}};
    int *cursor = values + 1;
    struct Point *point_cursor = points + 1;
    int index = 1;
    struct Index selector = {1};
    int total = 0;

    total += index[values];
    total += index[cursor];
    total += index[points].y;
    total += index[point_cursor].x;
    total += index[fixed];

    index[values] = 20;
    index[cursor] += 3;
    int *slot = &index[values];
    *slot += 2;

    index[points].x = 30;
    int *field = &index[points].y;
    *field = 40;
    index[point_cursor].y += 4;

    total += values[1];
    total += values[2];
    total += points[1].x;
    total += points[1].y;
    total += points[2].x;
    total += points[2].y;
    total += selector.value[values];
    selector.value[values] += 1;
    total += values[1];
    return total;
}
