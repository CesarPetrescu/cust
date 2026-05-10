struct Point { int x; int y; };
struct Box { struct Point inner; int tail; };

int sum_point(struct Point *point) {
    point->x += 3;
    point->y += 4;
    return point->x + point->y;
}

int main(void) {
    struct Point *inner = &((struct Box){{5, 7}, 9}).inner;
    int before = inner->x + inner->y;
    int after = sum_point(inner);
    inner->x = inner->x + 1;

    return before + after + inner->x + inner->y;
}
