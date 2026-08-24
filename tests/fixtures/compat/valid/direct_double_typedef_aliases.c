typedef double Real;
typedef Real Reading;
typedef const Real ConstReal;
typedef Reading Row[3];
typedef const Reading ConstRow[2];
typedef Real *RealPtr;
typedef const Real *ConstRealPtr;
typedef Real * const FixedRealPtr;

struct Sample {
    Reading scalar;
    Row values;
    RealPtr pointer;
    ConstReal fixed;
};

Reading adjust(Real value) {
    return value + 0.5;
}

RealPtr advance(RealPtr pointer) {
    return pointer + 1;
}

Real sum(Row values) {
    return values[0] + values[1] + values[2];
}

Real const_sum(ConstRow values) {
    return values[0] + values[1];
}

int main(void) {
    Row values = {1.0, 2.0, 3.0};
    ConstRow fixed_values = {4.0, 5.0};
    struct Sample sample = {6.0, {7.0, 8.0, 9.0}, values, 10.0};
    RealPtr cursor = advance(sample.values);
    ConstRealPtr view = fixed_values;
    FixedRealPtr fixed_pointer = values;

    *fixed_pointer += 0.25;
    cursor[1] += 0.5;

    return adjust(sample.scalar) == 6.5
            && sum(values) == 6.25
            && const_sum(fixed_values) == 9.0
            && sample.values[2] == 9.5
            && sample.pointer == values
            && view[1] == 5.0
            && sizeof(Row) == 3 * sizeof(Real)
            && _Alignof(ConstRow) == _Alignof(Reading)
            && sizeof(RealPtr) == sizeof(void *)
            && _Generic(sample.scalar, Real: 1, default: 0)
            && _Generic(cursor, RealPtr: 1, default: 0)
        ? 0 : 1;
}
