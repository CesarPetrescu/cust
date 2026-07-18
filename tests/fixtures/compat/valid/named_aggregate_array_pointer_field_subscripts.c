struct Point {
    int value;
};

struct Node {
    int *values;
    struct Point *points;
};

int main(void) {
    int values[3] = {2, 4, 6};
    struct Point points[2] = {{3}, {5}};
    struct Node nodes[1] = {{values, points}};
    int total = 0;

    total += nodes[0].values[1];
    total += 1[nodes[0].values];
    nodes[0].values[0] = 7;
    total += nodes[0].values[0];
    total += (nodes[0].values[1] += 2);
    total += nodes[0].values[2]++;
    total += ++nodes[0].values[2];

    int *slot = &nodes[0].values[0];
    *slot += 1;
    total += *slot;

    total += (nodes[0].points[1].value = 7);
    total += (nodes[0].points[0].value += 2);
    total += nodes[0].points[1].value++;
    total += ++1[nodes[0].points].value;

    struct Point *point_slot = &nodes[0].points[0];
    point_slot->value += 1;
    total += point_slot->value;

    total += sizeof(nodes[0].values[0]) == sizeof(int);
    total += sizeof(nodes[0].points[0]) == sizeof(struct Point);
    total += sizeof(nodes[0].points[0].value) == sizeof(int);
    return total;
}
