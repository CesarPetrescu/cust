int main(void) {
    int total = 0;

    int decl_ok = sizeof(struct DeclBox { int value; }) == sizeof(struct DeclBox),
        decl_value = 4;
    struct DeclBox decl_box = {decl_value};
    total += decl_ok + decl_box.value;

    int union_ok = sizeof(union DeclChoice { int value; char tag; }) == sizeof(union DeclChoice),
        union_value = 6;
    union DeclChoice decl_choice = {union_value};
    total += union_ok + decl_choice.value;

    total = total + (sizeof(struct AssignBox { int left; int right; }) == sizeof(struct AssignBox));
    struct AssignBox assign_box = {8, 9};
    total += assign_box.right;

    total += sizeof(union AssignChoice { int value; char tag; }) == sizeof(union AssignChoice);
    union AssignChoice assign_choice = {11};
    total += assign_choice.value;

    return total;
}
