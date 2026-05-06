const int global_limit = 7;
const char global_marker = 'A';
const int global_values[3];

int sum_const_param(const int value, const char marker) {
    int local = value + marker;
    return local;
}

int main() {
    const int local = 5;
    const char letter = 'B';
    const int values[3];
    int mutable = local + global_limit + values[0] + global_values[1];
    mutable += sum_const_param(3, 'C');
    return mutable + letter - global_marker;
}
