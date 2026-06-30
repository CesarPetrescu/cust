int main(void) {
    int values[5] = {4, 6, 8, 10, 12};
    int total = 0;
    int one = 1;
    int zero = 0;
    if ((values + (sizeof(struct PtrEq { int value; }) == sizeof(struct PtrEq))) == values + one) {
        total += values[1];
    }
    if ((values + 3) > (values + (sizeof(union PtrRel { int value; char tag; }) == sizeof(union PtrRel)) + zero)) {
        total += values[3];
    }
    total += (values + ((sizeof(struct PtrDiff { char tag; }) == sizeof(struct PtrDiff)) + 3)) - (values + one);
    int *field = &((struct PtrCmpField { int value; }){17}).value;
    if (field != 0) {
        total += *field;
    }
    struct PtrCmpField field_object = {29};
    return total + field_object.value;
}
