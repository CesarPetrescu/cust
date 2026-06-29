int main(void) {
    int total = 0;

    int *field = &((struct PointerBox { int value; }){5}).value;
    total += *field;
    struct PointerBox pointer_box = {3};
    total += pointer_box.value;

    field = &((struct AssignPointerBox { int value; }){7}).value;
    total += *field;
    struct AssignPointerBox assign_pointer_box = {11};
    total += assign_pointer_box.value;

    int *union_field = &((union PointerChoice { int value; char tag; }){13}).value;
    total += *union_field;
    union PointerChoice pointer_choice = {17};
    total += pointer_choice.value;

    return total;
}
