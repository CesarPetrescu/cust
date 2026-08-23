static double global_values[3] = {1.0, 2.0, 3.0};
static double global_value = 4.0;
static double *global_pointer = &global_value;

struct DoubleSlot {
    double *pointer;
};

double *advance(double *pointer) {
    return pointer + 1;
}

double sum_pair(const double *pointer) {
    return pointer[0] + pointer[1];
}

double *persistent(void) {
    static double value = 5.0;
    return &value;
}

int main(void) {
    double local = 6.0;
    double values[3] = {7.0, 8.0, 9.0};
    double *first = values;
    double *second = advance(first);
    const double *view = global_values;
    struct DoubleSlot slot = {&local};
    void *opaque = second;

    *global_pointer += 0.5;
    second[1] += 0.5;
    *slot.pointer += 0.25;
    *persistent() += 0.75;

    return global_value == 4.5 &&
           values[2] == 9.5 &&
           local == 6.25 &&
           *persistent() == 5.75 &&
           sum_pair(view) == 3.0 &&
           (double *)opaque == second &&
           second - first == 1 &&
           first < second &&
           sizeof(double *) > 0 &&
           _Alignof(double *) > 0 &&
           _Generic(second, double *: 1, default: 0)
               ? 0
               : 1;
}
