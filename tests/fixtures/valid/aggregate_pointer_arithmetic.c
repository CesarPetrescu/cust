struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int move_point_pointer(struct Point *p) {
    p = p + 1;
    p->x += 3;
    p++;
    return p->y;
}

int move_union_pointer(union Number *p) {
    ++p;
    p->value += 4;
    p -= 1;
    return (p + 1)->tag;
}

int point_distance(struct Point *first, struct Point *last) {
    return last - first;
}

int union_distance(union Number *first, union Number *last) {
    return last - first;
}

int main(void) {
    struct Point points[3] = {{1, 2}, {3, 4}, {5, 6}};
    union Number numbers[3] = {{7}, {8}, {9}};

    struct Point *p = &points[0];
    union Number *n = &numbers[0];

    int sum = 0;
    sum += move_point_pointer(p);
    sum += points[1].x;
    sum += move_union_pointer(n);
    sum += numbers[1].value;
    sum += point_distance(&points[0], &points[2]);
    sum += union_distance(&numbers[0], &numbers[2]);

    p = &points[2];
    p--;
    sum += p->y;

    n = &numbers[0];
    n += 2;
    sum += n->value;

    return sum;
}
