struct Point { int x; int y; };
union Number { int value; char tag; };

int global_values[3] = {1, 2, 3}, global_more[2];
struct Point global_point = {4, 5}, global_copy;
union Number global_number = {6}, global_other;

int main(void) {
    int values[3] = {7, 8, 9}, zeros[2];
    int *p = values, *q = values + 2;
    const int *view = values + 1, *start = values;
    struct Point point = {10, 11}, copy = point;
    struct Point points[2] = {{1, 2}, {3, 4}}, empty_points[1];
    union Number number = {12}, other;
    union Number numbers[2] = {{13}, {14}}, empty_numbers[1];

    global_copy = global_point;
    global_more[1] = 15;
    global_other.value = 16;
    zeros[0] = 17;
    zeros[1] = q - p;
    copy.x = copy.x + *view + *start;
    empty_points[0].x = points[1].x;
    empty_points[0].y = points[1].y;
    other.value = number.value + numbers[0].value + numbers[1].value;
    empty_numbers[0].value = other.value;

    return global_values[0]
        + global_more[1]
        + global_copy.y
        + global_other.value
        + zeros[0]
        + zeros[1]
        + copy.x
        + points[0].y
        + empty_points[0].x
        + empty_numbers[0].value;
}
