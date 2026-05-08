struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

struct Point pick_point(struct Point *p) {
    return *p;
}

int sum_point(struct Point p) {
    return p.x + p.y;
}

void overwrite_point(struct Point *dst, struct Point src) {
    *dst = src;
}

union Number pick_number(union Number *p) {
    return *p;
}

void overwrite_number(union Number *dst, union Number src) {
    *dst = src;
}

int main(void) {
    struct Point a = {2, 3};
    struct Point b = {10, 20};
    struct Point *pa = &a;
    struct Point copy = *pa;
    int total = sum_point(copy);

    overwrite_point(pa, b);
    total += a.x + a.y;
    copy = *pa;
    total += sum_point(copy);
    total += sum_point(*pa);
    total += sum_point(pick_point(pa));

    union Number n = {7};
    union Number m = {11};
    union Number *pn = &n;
    union Number number_copy = *pn;
    total += number_copy.value;

    overwrite_number(pn, m);
    total += n.value;
    number_copy = *pn;
    total += number_copy.value;
    number_copy = pick_number(pn);
    total += number_copy.value;

    return total;
}
