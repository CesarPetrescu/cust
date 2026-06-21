struct Point { int x; int y; };
union Number { int value; char tag; };

int sum_points(struct Point *left, struct Point *right) {
    return left->x + right->y + left[1].x;
}

int main(void) {
    struct Point points[3] = {{1, 2}, {3, 4}, {5, 6}};
    union Number numbers[3] = {{7}, {8}, {9}};
    struct Point *p = points, *q = points + 1;
    union Number *n = numbers, *m = numbers + 2;

    p->x += 10;
    q[1].y = p[0].y + q->x;
    m->value += n[1].value;

    return p->x
        + q->y
        + q[1].y
        + n->value
        + m->value
        + sum_points(p, q);
}
