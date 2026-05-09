int bump(int *values) {
    2[values] = 9;
    return 0[values] + 1[values] + 2[values];
}

struct Point {
    int x;
    int y;
};

int sum_points(struct Point points[]) {
    1[points].y = 8;
    return 0[points].x + 1[points].x + 1[points].y;
}

int main(void) {
    int values[3] = {4, 5, 6};
    int *p = values;
    char *text = "cat";
    struct Point points[2] = {{1, 2}, {3, 4}};

    int total = bump(values);
    total = total + 0[p] + 1[p] + 2[p];
    total = total + 1["hi"] + 2[text];
    total = total + 1[(int[]){7, 8, 9}];
    total = total + sum_points(points);
    return total - 248;
}
