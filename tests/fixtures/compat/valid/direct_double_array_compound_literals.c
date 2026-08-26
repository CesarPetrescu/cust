typedef double Real;
typedef Real Row[4];
typedef const Real ConstRow[3];

int marker;

Real marked(Real value) {
    marker += 1;
    return value;
}

Real *advance(Real *pointer) {
    return pointer + 1;
}

int main(void) {
    Real *direct = (double[4]){marked(1.25), [2] = 3.5};
    Real *inferred = (double[]){[1] = 2.25, marked(4.5)};
    Real *aliased = (Row){5.0, 6.0};
    const Real *view = (ConstRow){7.0, [2] = 9.0};

    advance(direct)[0] += 0.75;
    aliased[3] = 8.25;

    return marker == 2
            && direct[0] == 1.25 && direct[1] == 0.75
            && direct[2] == 3.5 && direct[3] == 0.0
            && inferred[0] == 0.0 && inferred[1] == 2.25
            && inferred[2] == 4.5
            && aliased[0] == 5.0 && aliased[1] == 6.0
            && aliased[2] == 0.0 && aliased[3] == 8.25
            && view[0] == 7.0 && view[1] == 0.0 && view[2] == 9.0
            && sizeof((double[2]){1.0}) == 2 * sizeof(double)
            && sizeof((Row){1.0}) == sizeof(Row)
            && sizeof(((double[]){1.0})[0]) == sizeof(double)
            && _Alignof(Row) == _Alignof(Real)
            && _Generic((double[]){1.0}, double *: 1, default: 0)
        ? 0 : 1;
}
