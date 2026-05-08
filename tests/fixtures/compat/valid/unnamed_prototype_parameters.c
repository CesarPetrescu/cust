struct Point { int x; int y; };
union Number { int value; char tag; };

typedef int *IntPtr;

int add(int, int);
char pick_char(char *);
void bump_all(int [], struct Point [], union Number [], IntPtr);
struct Point choose_point(struct Point *, int);
union Number choose_number(union Number [], int);

int main(void) {
    int values[2] = {3, 4};
    struct Point points[2] = {{1, 2}, {5, 6}};
    union Number numbers[2] = {{7}, {8}};
    int extra = 1;

    bump_all(values, points, numbers, &extra);
    struct Point p = choose_point(points, 1);
    union Number n = choose_number(numbers, 0);

    return add(values[0], values[1])
        + pick_char("az")
        + p.x
        + p.y
        + n.value
        + extra
        - 'a';
}

int add(int left, int right) {
    return left + right;
}

char pick_char(char *text) {
    return text[0];
}

void bump_all(int values[], struct Point points[], union Number numbers[], IntPtr extra) {
    values[0] += 1;
    values[1] += 2;
    points[1].x += 3;
    points[1].y += 4;
    numbers[0].value += 5;
    *extra += 6;
}

struct Point choose_point(struct Point *points, int index) {
    return points[index];
}

union Number choose_number(union Number numbers[], int index) {
    return numbers[index];
}
