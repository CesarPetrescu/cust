typedef struct Point {
    int x;
    int y;
} Point, *PointPtr, PointArray[3];

typedef union Number {
    int value;
    char tag;
} Number, *NumberPtr, NumberArray[2];

int sum_points(PointArray points, int len) {
    PointPtr cursor = points;
    int total = 0;
    for (int i = 0; i < len; i = i + 1) {
        total = total + cursor[i].x + cursor[i].y;
    }
    return total;
}

int main(void) {
    PointArray points = {{1, 2}, {3, 4}, {5, 6}};
    PointPtr middle = points + 1;
    middle->x += 10;

    NumberArray numbers = {{8}, {9}};
    NumberPtr picked = numbers;
    picked[1].value += 4;

    return sum_points(points, 3) + middle->x + (sizeof(PointArray) == 3 * sizeof(Point)) + picked[1].value;
}
