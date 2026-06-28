int main(void) {
    int enum_sized[sizeof(enum LengthTag { LENGTH_VALUE = 5 })];
    enum_sized[0] = LENGTH_VALUE;

    typedef int CellSized[sizeof(struct Cell { char tag; })];
    struct Cell cell = {'A'};
    CellSized cells = {cell.tag - 'A' + 2};

    int array_type_check =
        sizeof(int[sizeof(enum ArrayLengthTag { ARRAY_LENGTH_VALUE = 3 })]) ==
        sizeof(int) * sizeof(enum ArrayLengthTag);

    int aggregate_array_type_check =
        sizeof(struct Box { struct Cell inner; char tail; }[2]) ==
        2 * sizeof(struct Box);
    struct Box box = {{'B'}, 'C'};

    return enum_sized[0] + cells[0] + ARRAY_LENGTH_VALUE + array_type_check +
           aggregate_array_type_check + box.inner.tag - 'A' + box.tail - 'A';
}
