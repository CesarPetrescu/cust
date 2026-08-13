double global_values[] = {1.0, [2] = 3.5};

int main(void) {
    static double persistent[] = {1.5, 2.5};
    const double fixed[4] = {1.25, 2.0, [3] = 4.75};
    double values[] = {1.25, 2.0};
    double assigned = (values[0] = 3.0);
    double compounded = (values[1] += 0.5);
    double post = values[0]++;
    double pre = --values[1];
    int index = 0;
    double reverse_read = index[values];
    double reverse_post = index[values]++;
    double reverse_pre = ++index[values];

    index[values] += 0.25;
    global_values[1] = 2.25;
    persistent[0] *= 2.0;

    return fixed[0] == 1.25
                && fixed[1] == 2.0
                && fixed[2] == 0.0
                && fixed[3] == 4.75
                && assigned == 3.0
                && compounded == 2.5
                && post == 3.0
                && reverse_read == 4.0
                && reverse_post == 4.0
                && reverse_pre == 6.0
                && index[values] == 6.25
                && _Generic(index[values], double: 1, default: 0)
                && values[0] == 6.25
                && pre == 1.5
                && values[1] == 1.5
                && global_values[0] == 1.0
                && global_values[1] == 2.25
                && global_values[2] == 3.5
                && persistent[0] == 3.0
                && sizeof(values) == 2 * sizeof(double)
                && sizeof(values[0]) == sizeof(double)
                && sizeof(double[3]) == 3 * sizeof(double)
                && _Alignof(double[3]) == _Alignof(double)
           ? 0
           : 1;
}
