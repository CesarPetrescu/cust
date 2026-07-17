struct Point {
    int x;
};

struct Holder {
    int *primary;
};

struct Box {
    struct Point point;
    struct Point points[1];
};

int marker = 0;
int values[4] = {1, 2, 3, 4};

int scalar_call(void) {
    marker++;
    return 7;
}

int *forward(int *value) {
    return value;
}

struct Point make_point(void) {
    marker++;
    return (struct Point){7};
}

int main(void) {
    int scalar = 3;
    int *cursor = values;
    struct Point point = {3};
    struct Point target = {3};
    struct Point replacement = {5};
    struct Point *slot = &target;
    struct Holder holder = {values};
    struct Holder *holder_view = &holder;
    struct Point points[1] = {{4}};
    struct Box box = {{4}, {{6}}};
    int scalar_result = 0;
    int *pointer_result = values;
    struct Point aggregate_result = {0};
    int total = 0;

    int scalar_decl = (marker++, scalar);
    total += scalar_decl;
    int *pointer_decl = (marker++, cursor + 1);
    total += *pointer_decl;
    struct Point aggregate_decl = (marker++, point);
    total += aggregate_decl.x;

    scalar_result = scalar = (marker++, 5);
    total += scalar_result;
    pointer_result = cursor = (marker++, values + 2);
    total += *pointer_result;
    aggregate_result = target = (marker++, replacement);
    total += aggregate_result.x;

    int scalar_cond = 1 ? (marker++, 8) : (marker += 20, 9);
    total += scalar_cond;
    int *pointer_call = (marker++, forward(values + 3));
    total += *pointer_call;
    struct Point aggregate_call = make_point();
    total += aggregate_call.x;

    scalar_result = (int)(marker++, 9);
    total += scalar_result;
    pointer_result = holder_view->primary = (marker++, values + 1);
    total += *pointer_result;
    aggregate_result = box.points[0] = (marker++, (struct Point){9});
    total += aggregate_result.x;

    return marker * 10 + total + (slot == &target) + (points[0].x == 4) - 2;
}
