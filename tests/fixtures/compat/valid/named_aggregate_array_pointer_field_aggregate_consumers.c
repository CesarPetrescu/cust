struct Point {
    int value;
    int extra;
};

struct Node {
    struct Point *cursor;
};

int read_value(struct Point point) {
    point.value += 10;
    return point.value;
}

struct Point return_read(void) {
    struct Point points[1] = {{3, 4}};
    struct Node nodes[1] = {{points}};
    return nodes[0].cursor[0];
}

struct Point return_assignment(void) {
    struct Point points[1] = {{1, 2}};
    struct Point replacement = {5, 6};
    struct Node nodes[1] = {{points}};
    return nodes[0].cursor[0] = replacement;
}

int main(void) {
    struct Point points[2] = {{7, 8}, {9, 10}};
    struct Point replacement = {11, 12};
    struct Node nodes[1] = {{points}};
    int total = 0;

    total += read_value(nodes[0].cursor[0]);

    struct Point direct = nodes[0].cursor[1];
    direct.value = 20;
    total += points[1].value + direct.value;

    struct Point returned = return_read();
    returned.value = 21;
    total += returned.extra + returned.value;

    struct Point assigned_return = return_assignment();
    total += assigned_return.value + assigned_return.extra;

    struct Point conditional = 1 ? nodes[0].cursor[0] : replacement;
    conditional.value = 30;
    total += points[0].value + conditional.value;

    int marker = 0;
    struct Point comma_copy = (marker += 1, nodes[0].cursor[1]);
    comma_copy.value = 31;
    total += marker + points[1].value + comma_copy.value;

    int outer = 0;
    int inner = 0;
    (void)nodes[outer++].cursor[inner++];
    total += outer + inner;

    outer = 0;
    inner = 0;
    int rhs = 0;
    (void)(nodes[outer++].cursor[inner++] = (rhs++, replacement));
    total += outer + inner + rhs + points[0].value + points[0].extra;

    total += (nodes[0].cursor[1] = replacement).extra;
    total += sizeof(nodes[0].cursor[0]) == sizeof(struct Point);
    return total;
}
