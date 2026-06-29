int main(void) {
    int total = 0;

    (void)(sizeof(struct VoidBox { int value; }) == sizeof(struct VoidBox));
    struct VoidBox void_box = {5};
    total += void_box.value;

    (void)((struct LiteralBox { int value; }){7}).value;
    struct LiteralBox literal_box = {11};
    total += literal_box.value;

    (void)(sizeof(union VoidChoice { int value; char tag; }) == sizeof(union VoidChoice));
    union VoidChoice void_choice = {13};
    total += void_choice.value;

    return total;
}
