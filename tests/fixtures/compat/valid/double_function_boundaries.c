double add_half(double);

double add_half(double value) {
    return value + 0.5;
}

double from_int(int value) {
    return value;
}

double recurse(double value, int depth) {
    return depth ? recurse(value + 0.25, depth - 1) : value;
}

int main(void) {
    int marker = 0;
    double result = add_half((marker += 1, 2));

    return result == 2.5
                && marker == 1
                && from_int(3) == 3.0
                && recurse(1.0, 4) == 2.0
                && (int)add_half(3.75) == 4
                && _Generic(add_half(1), double: 1, default: 0)
                && sizeof(add_half(marker)) == sizeof(double)
                && marker == 1
           ? 0
           : 1;
}
