union Number {
    int value;
    char tag;
};

union Node {
    int value;
    union Node *next;
};

union External {
    int *external;
    int marker;
};

int bump(union Number *p) {
    p->value += 3;
    return p->tag;
}

int main(void) {
    union Number n = {4};
    union Number *p = &n;
    int sum = p->value + (*p).tag;
    p->tag = 6;
    sum = sum + n.value + bump(&n);

    union Number values[2] = {{1}, {2}};
    union Number *vp = &values[1];
    vp->value += 5;
    sum = sum + values[1].tag;

    union Node a = {10};
    union Node b = {20};
    a.next = &b;
    a.next->value += 1;
    sum = sum + b.value;

    int external = 8;
    union External e;
    e.external = &external;
    *e.external += 2;
    sum = sum + external;

    return sum;
}
