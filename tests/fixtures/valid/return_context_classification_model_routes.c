struct Point {
    int x;
};

int marker = 0;
int values[4] = {1, 2, 3, 4};

int scalar_assignment(void) {
    int value = 3;
    return value = (marker++, 5);
}

int scalar_conditional(void) {
    return 1 ? (marker++, 8) : (marker += 20, 9);
}

int *pointer_assignment(void) {
    int *cursor = values;
    return cursor = (marker++, values + 2);
}

int *pointer_conditional(void) {
    return 1 ? (marker++, values + 1) : (marker += 20, values);
}

struct Point aggregate_assignment(void) {
    struct Point point = {3};
    struct Point replacement = {5};
    return point = (marker++, replacement);
}

struct Point aggregate_conditional(void) {
    struct Point point = {6};
    struct Point replacement = {7};
    return 1 ? (marker++, replacement) : (marker += 20, point);
}

struct Point make_point(void) {
    marker++;
    return (struct Point){9};
}

struct Point aggregate_call(void) {
    return make_point();
}

void touch(void) {
    marker++;
    return;
}

int main(void) {
    int scalar_one = scalar_assignment();
    int scalar_two = scalar_conditional();
    int *pointer_one = pointer_assignment();
    int *pointer_two = pointer_conditional();
    struct Point aggregate_one = aggregate_assignment();
    struct Point aggregate_two = aggregate_conditional();
    struct Point aggregate_three = aggregate_call();
    touch();

    return marker * 20 + scalar_one + scalar_two + *pointer_one + *pointer_two +
           aggregate_one.x + aggregate_two.x + aggregate_three.x;
}
