int sum_static(int values[static 3]) {
    values[2] = values[2] + 1;
    return values[0] + values[2];
}

int sum_restrict(int values[restrict 3]) {
    return values[1] + values[2];
}

int sum_static_const_view(const int values[static const 3]) {
    return values[0] + values[1] + values[2];
}

struct Point {
    int x;
    int y;
};

int sum_points(struct Point points[static 2]) {
    points[1].y = points[1].y + 3;
    return points[0].x + points[1].y;
}

int main(void) {
    int values[3] = {2, 4, 6};
    struct Point points[2] = {{1, 2}, {3, 4}};
    int total = sum_static(values);
    total = total + sum_restrict(values);
    total = total + sum_static_const_view(values);
    return total + sum_points(points);
}
