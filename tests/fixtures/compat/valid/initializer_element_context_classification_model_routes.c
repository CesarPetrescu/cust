struct Point {
    int x;
};

struct Holder {
    int scalars[2];
    int *pointer;
    struct Point point;
    struct Point points[2];
};

struct ScalarHolder {
    int scalar;
};

int marker = 0;
int values[4] = {2, 4, 6, 8};

int bump_scalar(void) {
    marker++;
    return 5;
}

int *forward(int *value) {
    return value;
}

struct Point make_point(void) {
    marker++;
    return (struct Point){7};
}

int main(void) {
    int root_positional[2] = {(marker++, 3), 4};
    int root_designated[2] = {[1] = 6, [0] = (marker++, 5)};
    struct ScalarHolder field_positional = {1};
    struct ScalarHolder field_designated = {.scalar = 2};

    struct Holder scalar_positional = {
        {(marker++, 7), 8},
        values,
        {9},
        {{10}, {11}},
    };
    struct Holder scalar_designated = {
        .scalars = {[1] = 13, [0] = (marker++, 12)},
        .pointer = values + 1,
        .point = {14},
        .points = {[1] = {16}, [0] = {15}},
    };

    struct Holder pointer_route = {
        {0, 0},
        (marker++, forward(values + 2)),
        {0},
        {{0}, {0}},
    };
    struct Point replacement = {17};
    struct Holder aggregate_route = {
        {0, 0},
        values,
        (marker++, replacement),
        {{0}, {0}},
    };

    struct Point root_points[2] = {{(marker++, 18)}, [1] = {19}};
    struct Holder embedded_route = {
        .scalars = {0, 0},
        .pointer = values,
        .point = {0},
        .points = {{(marker++, 20)}, [1] = {21}},
    };

    int call_array[1] = {bump_scalar()};
    struct Holder call_holder = {
        {0, 0},
        values,
        make_point(),
        {{0}, {0}},
    };

    int total = root_positional[0] + root_positional[1]
        + root_designated[0] + root_designated[1]
        + field_positional.scalar + field_designated.scalar
        + scalar_positional.scalars[0] + scalar_positional.scalars[1]
        + *scalar_positional.pointer + scalar_positional.point.x
        + scalar_positional.points[0].x + scalar_positional.points[1].x
        + scalar_designated.scalars[0] + scalar_designated.scalars[1]
        + *scalar_designated.pointer + scalar_designated.point.x
        + scalar_designated.points[0].x + scalar_designated.points[1].x
        + *pointer_route.pointer + aggregate_route.point.x
        + root_points[0].x + root_points[1].x
        + embedded_route.points[0].x + embedded_route.points[1].x
        + call_array[0] + call_holder.point.x;

    return total + (marker == 10) - 1;
}
