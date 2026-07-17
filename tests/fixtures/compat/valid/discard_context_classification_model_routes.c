struct Point {
    int x;
};

struct Holder {
    int *primary;
    struct Point point;
};

int marker = 0;

int scalar_call(void) {
    marker++;
    return 7;
}

void touch(void) {
    marker++;
}

void touch_noop(void) {
    return;
}

int *forward(int *value) {
    return value;
}

struct Point make_point(void) {
    struct Point result = {5};
    return result;
}

int main(void) {
    int values[4] = {1, 2, 3, 4};
    int *cursor = values;
    int scalar = scalar_call();
    struct Point point = {3};
    struct Point replacement = {7};
    struct Point *point_slot = &point;
    struct Holder holder = {values, {4}};
    struct Holder *view = &holder;

    (void)(scalar = (marker++, 7));
    (void)(scalar += (marker++, 2));
    (void)(cursor = (marker++, values + 1));
    (void)(cursor += (marker++, 1));
    (void)(holder.primary = (marker++, values + 1));
    (void)(view->primary = (marker++, values + 2));
    (void)(((struct Holder){values, {4}}).primary = (marker++, values + 1));
    (void)(marker++, forward(values + 1));
    (void)(marker++, (1 ? values + 1 : cursor));
    (void)(marker++, (int *)(values + 1));
    (void)(marker++, point);
    (void)(point = (marker++, replacement));
    (void)(marker++, (1 ? point : replacement));
    (void)(marker++, make_point());
    (void)(marker++, (struct Point){9});
    (void)(marker++, holder.point);
    (void)(*(marker++, point_slot) = replacement);
    touch();
    (marker++, touch_noop());
    (1 ? touch() : touch_noop());
    (marker++, (void)scalar);

    return marker * 10 + scalar + (cursor - values) +
           (holder.primary - values) + point.x;
}
