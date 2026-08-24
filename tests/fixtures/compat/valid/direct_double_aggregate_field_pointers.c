struct Item {
    double scalar;
    double values[3];
};

struct Box {
    struct Item items[2];
};

struct PointerBox {
    double *pointer;
};

double *advance(double *pointer) {
    return pointer + 1;
}

double second(const double *pointer) {
    return pointer[1];
}

struct Item make(double base) {
    return (struct Item){base, {base + 1.0, base + 2.0, base + 3.0}};
}

int main(void) {
    struct Item items[2] = {
        {1.0, {2.0, 3.0, 4.0}},
        {5.0, {6.0, 7.0, 8.0}}
    };
    struct Item *arrow = items;
    struct Box box = {{
        {9.0, {10.0, 11.0, 12.0}},
        {13.0, {14.0, 15.0, 16.0}}
    }};
    struct Box *box_pointer = &box;
    const struct Item fixed = {17.0, {18.0, 19.0, 20.0}};
    double *indexed = &items[1].scalar;
    double *reverse = &1[items].scalar;
    double *nested = advance(box.items[1].values);
    double *nested_arrow = box_pointer->items[0].values + 2;
    double *literal_scalar = &((struct Item){21.0, {22.0, 23.0, 24.0}}).scalar;
    double *literal_array = ((struct Item){25.0, {26.0, 27.0, 28.0}}).values;
    struct PointerBox forwarded = {advance(arrow->values)};
    const void *opaque = fixed.values;
    const double *qualified_round_trip = opaque;

    *indexed += 0.5;
    forwarded.pointer[0] += 1.0;

    return indexed == reverse && *indexed == 5.5
            && nested - box.items[1].values == 1 && *nested == 15.0
            && *nested_arrow == 12.0 && *literal_scalar == 21.0
            && literal_array[1] == 27.0 && arrow->values[1] == 4.0
            && qualified_round_trip[1] == 19.0
            && second(make(29.0).values) == 31.0
            && sizeof(&items[0].scalar) == sizeof(double *)
            && _Alignof(double *) == _Alignof(void *)
            && _Generic(box.items[0].values, double *: 1, default: 0)
        ? 0 : 1;
}
