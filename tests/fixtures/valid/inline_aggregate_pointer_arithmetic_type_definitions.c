int main(void) {
    int values[6] = {2, 3, 5, 7, 11, 13};
    int *cursor = values + (sizeof(struct PtrOffset { int value; }) == sizeof(struct PtrOffset));
    int selected = cursor[sizeof(union PtrIndex { int value; char tag; }) == sizeof(union PtrIndex)];
    int *field = &((struct PtrField { int value; }){17}).value;
    *field += values[(sizeof(struct PtrReverse { char tag; }) == sizeof(struct PtrReverse)) + 2];
    struct PtrOffset offset = {19};
    union PtrIndex index = {23};
    struct PtrField point = {29};
    struct PtrReverse reverse = {'A'};
    return *cursor + selected + *field + offset.value + index.value + point.value + reverse.tag;
}
