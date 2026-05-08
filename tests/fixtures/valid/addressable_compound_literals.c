struct Point { int x; int y; };

int bump(int *slot) {
    *slot += 4;
    return *slot;
}

void move_point(struct Point *point) {
    point->x += 5;
    point->y += 6;
}

int main(void) {
    int *slot = &(int){7};
    int scalar = bump(slot);
    *slot += 1;

    struct Point *point = &(struct Point){.x = 2, .y = 3};
    move_point(point);
    point->x += 1;

    int *other = &(int){1};
    *other = *slot + point->y;

    return scalar + *slot + point->x + point->y + *other;
}
