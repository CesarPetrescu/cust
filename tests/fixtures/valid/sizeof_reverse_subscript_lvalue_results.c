struct Point {
    int x;
    char tag;
};

struct Index {
    int value;
};

int main(void) {
    int values[3] = {3, 5, 7};
    char bytes[3] = {'a', 'b', 'c'};
    struct Point points[3] = {{1, 'A'}, {2, 'B'}, {3, 'C'}};
    int *cursor = values;
    struct Point *point_cursor = points;
    int index = 1;
    struct Index selector = {1};
    int pointer_marker = 0;
    int rhs_marker = 0;
    int checks = 0;

    checks += sizeof(index[(pointer_marker += 1, values)]
        = (rhs_marker += 1, 9)) == sizeof(values[index]);
    checks += sizeof(index[(pointer_marker += 1, bytes)]
        += (rhs_marker += 1, 2)) == sizeof(bytes[index]);
    checks += sizeof(++index[(pointer_marker += 1, cursor)]) == sizeof(cursor[index]);

    checks += sizeof(index[(pointer_marker += 1, points)].tag
        = (rhs_marker += 1, 'X')) == sizeof(points[index].tag);
    checks += sizeof(index[(pointer_marker += 1, point_cursor)].x
        += (rhs_marker += 1, 2)) == sizeof(point_cursor[index].x);
    checks += sizeof(++index[(pointer_marker += 1, points)].tag)
        == sizeof(points[index].tag);

    checks += sizeof(selector.value[(pointer_marker += 1, values)]
        = (rhs_marker += 1, 11)) == sizeof(values[selector.value]);
    checks += sizeof(selector.value[(pointer_marker += 1, bytes)]
        += (rhs_marker += 1, 3)) == sizeof(bytes[selector.value]);
    checks += sizeof(++selector.value[(pointer_marker += 1, cursor)])
        == sizeof(cursor[selector.value]);

    checks += sizeof(selector.value[(pointer_marker += 1, points)].tag
        = (rhs_marker += 1, 'Y')) == sizeof(points[selector.value].tag);
    checks += sizeof(selector.value[(pointer_marker += 1, point_cursor)].x
        += (rhs_marker += 1, 4)) == sizeof(point_cursor[selector.value].x);

    checks += pointer_marker == 0;
    checks += rhs_marker == 0;
    return checks;
}
