struct Point {
    int x;
};

struct Box {
    struct Point point;
    struct Point points[1];
};

int marker = 0;
int values[4] = {1, 2, 3, 4};

int consume_scalar(int value) {
    return value;
}

int consume_pointer(int *value) {
    return *value;
}

int consume_point(struct Point value) {
    return value.x;
}

struct Point make_point(void) {
    marker++;
    return (struct Point){6};
}

int main(void) {
    int scalar = 3;
    int *cursor = values;
    struct Point target = {3};
    struct Point replacement = {5};
    struct Point *slot = &target;
    struct Point points[1] = {{4}};
    struct Box box = {{4}, {{6}}};
    struct Box *view = &box;
    int total = 0;

    total += consume_scalar(scalar = (marker++, 5));
    total += consume_scalar(1 ? (marker++, 8) : (marker += 20, 9));
    total += consume_pointer(cursor = (marker++, values + 2));
    total += consume_pointer(1 ? (marker++, values + 1) : (marker += 20, values));
    total += consume_point(target = (marker++, replacement));
    total += consume_point(*slot = (marker++, (struct Point){7}));
    total += consume_point(box.point = (marker++, replacement));
    total += consume_point(view->point = (marker++, (struct Point){8}));
    total += consume_point(points[0] = (marker++, replacement));
    total += consume_point(box.points[0] = (marker++, (struct Point){9}));
    total += consume_point(make_point());

    return marker * 10 + total;
}
