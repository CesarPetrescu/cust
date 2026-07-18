struct Point {
    int value;
};

struct Node {
    int *cursor;
    struct Point *point;
    const int *reader;
};

int main(void) {
    int values[5] = {2, 4, 6, 8, 10};
    struct Point points[3] = {{1}, {3}, {5}};
    struct Node nodes[2] = {
        {values, points, values + 4},
        {values + 1, points + 1, values + 3},
    };
    int ok = 0;

    int *read = nodes[0].cursor;
    int *replacement = (nodes[0].cursor = values + 1);
    int *added = (nodes[0].cursor += 2);
    int *subtracted = (nodes[0].cursor -= 1);
    int *post = nodes[0].cursor++;
    int *pre = --nodes[0].cursor;

    struct Point *point_read = nodes[1].point;
    struct Point *point_replacement = (nodes[1].point = points);
    struct Point *point_added = (nodes[1].point += 2);
    struct Point *point_subtracted = (nodes[1].point -= 1);
    struct Point *point_post = nodes[1].point++;
    struct Point *point_pre = --nodes[1].point;

    const int *reader = nodes[0].reader;

    ok += *read == 2;
    ok += *replacement == 4;
    ok += *added == 8;
    ok += *subtracted == 6;
    ok += *post == 6;
    ok += *pre == 6;
    ok += *nodes[0].cursor == 6;

    ok += point_read->value == 3;
    ok += point_replacement->value == 1;
    ok += point_added->value == 5;
    ok += point_subtracted->value == 3;
    ok += point_post->value == 3;
    ok += point_pre->value == 3;
    ok += nodes[1].point->value == 3;

    ok += *reader == 10;
    ok += sizeof(nodes[0].cursor) == sizeof(int *);
    ok += sizeof(nodes[1].point) == sizeof(struct Point *);

    return ok;
}
