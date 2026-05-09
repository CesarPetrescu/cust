union Number {
    int value;
    char tag;
};

union Number make_number(int value) {
    union Number number = {value};
    return number;
}

int main(void) {
    union Number left = {1};
    union Number right = {2};
    union Number replacement = {3};
    int total = ((union Number){4}).value;
    total = total + (left = right).value;
    total = total + (1 ? left : replacement).value;
    int marker = 0;
    total = total + (marker = marker + 1, replacement).value;
    total = total + marker;
    total = total + make_number(5).value;
    total = total + ((*(&left) = make_number(6))).value;
    return total;
}
