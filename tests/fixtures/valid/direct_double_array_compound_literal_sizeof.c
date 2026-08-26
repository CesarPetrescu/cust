int marker;

double marked(double value) {
    marker += 1;
    return value;
}

int main(void) {
    int full_object = sizeof((double[3]){marked(1.0), 2.0, 3.0})
        == 3 * sizeof(double);
    int element = sizeof(((double[]){marked(4.0)})[0]) == sizeof(double);
    return full_object && element && marker == 0 ? 0 : 1;
}
