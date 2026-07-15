struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

typedef const struct Point ConstPoints[4];
typedef const union Number ConstNumbers[4];

int main(void) {
    struct Point *mutable_points = (struct Point[4]){{11}, {12}, {13}, {14}};
    union Number *mutable_numbers = (union Number[4]){{21}, {22}, {23}, {24}};
    const struct Point *const_points = (ConstPoints){{31}, {32}, {33}, {34}};
    const union Number *const_numbers = (ConstNumbers){{41}, {42}, {43}, {44}};

    struct Point *p0 = mutable_points + 1;
    struct Point *p1 = 0 ? mutable_points + 3 : mutable_points + 2;
    int marker = 0;
    struct Point *p2 =
        (marker += (mutable_points + 3) - (mutable_points + 1), mutable_points + 3);
    const struct Point *cp = const_points + 2;
    union Number *n0 = mutable_numbers + 1;
    const union Number *cn = const_numbers + 2;

    int differences = (mutable_points + 3) - mutable_points;
    differences += (const_points + 2) - const_points;
    differences += (mutable_numbers + 3) - mutable_numbers;
    differences += (const_numbers + 2) - const_numbers;
    int comparisons = p0 == mutable_points + 1;
    comparisons += p2 > p0;
    comparisons += n0 == mutable_numbers + 1;
    comparisons += const_numbers + 2 > const_numbers;

    mutable_points[0].value = 5;
    mutable_numbers[0].value = 6;

    return p0->value + p1[0].value + p2->value + cp[0].value + n0->value +
           cn[0].value + differences + comparisons + mutable_points[0].value +
           mutable_numbers[0].value + marker;
}