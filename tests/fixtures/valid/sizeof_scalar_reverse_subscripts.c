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
    int marker = 0;
    int checks = 0;

    checks += sizeof(index[values]) == sizeof(values[index]);
    checks += sizeof(index[bytes]) == sizeof(bytes[index]);
    checks += sizeof(index[cursor]) == sizeof(cursor[index]);
    checks += sizeof(index[points]) == sizeof(points[index]);
    checks += sizeof(index[point_cursor]) == sizeof(point_cursor[index]);
    checks += sizeof(index[points].tag) == sizeof(points[index].tag);
    checks += sizeof(index[point_cursor].tag) == sizeof(point_cursor[index].tag);

    checks += sizeof(selector.value[values]) == sizeof(values[selector.value]);
    checks += sizeof(selector.value[cursor]) == sizeof(cursor[selector.value]);
    checks += sizeof(selector.value[points]) == sizeof(points[selector.value]);
    checks += sizeof(selector.value[point_cursor]) == sizeof(point_cursor[selector.value]);
    checks += sizeof(selector.value[points].tag) == sizeof(points[selector.value].tag);
    checks += sizeof(selector.value[point_cursor].tag)
        == sizeof(point_cursor[selector.value].tag);

    checks += sizeof((index += 1)[(marker += 10, cursor)]) == sizeof(cursor[index]);
    checks += index == 1;
    checks += marker == 0;
    return checks;
}