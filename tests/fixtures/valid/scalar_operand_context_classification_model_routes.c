struct Point {
    int x;
};

int marker = 0;

int scalar_call(void) {
    marker++;
    return 1;
}

int main(void) {
    int scalar = 0;
    int values[4] = {2, 4, 6, 8};
    int right[4] = {1, 3, 5, 7};
    struct Point points[4] = {{10}, {20}, {30}, {40}};

    int direct = values[1];
    int conditional = values[1 ? (marker++, 1) : (marker += 20, 0)];
    int reverse = (marker++, 1)[values];
    int assigned = values[scalar = (marker++, 1)];
    int called = values[scalar_call()];
    int casted = values[(int)(marker++, 1)];
    int differenced = values[(marker++, (right + 1) - right)];

    int aggregate = points[1 ? (marker++, 1) : (marker += 20, 0)].x;
    int reverse_aggregate = (marker++, 1)[points].x;
    int offset = *(values + (marker++, 1));
    int reverse_offset = *((marker++, 1) + values);
    int *cursor = values + 1;
    int negative_offset = *(cursor - (marker++, 1));

    int switched = 0;
    switch ((marker++, 1)) {
        case 1:
            switched = 7;
            break;
        default:
            switched = 9;
    }

    _Static_assert(1 ? 1 : 0, "conditional scalar constant");
    _Static_assert((int)1, "cast scalar constant");
    _Static_assert(sizeof(int[2]) == 2 * sizeof(int), "array size relationship");

    return direct + conditional + reverse + assigned + called + casted + differenced
        + aggregate + reverse_aggregate + offset + reverse_offset + negative_offset
        + switched + marker;
}
