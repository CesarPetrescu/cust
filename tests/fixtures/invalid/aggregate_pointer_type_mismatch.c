struct Point {
    int x;
};

union Number {
    int value;
};

void set_point(struct Point *p) {
    p->x = 1;
}

int main(void) {
    union Number n = {0};
    set_point(&n);
    return 0;
}
