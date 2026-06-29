int main(void) {
    int total = 0;
    int flag = 1;

    total += flag
        ? (sizeof(struct ThenBox { int value; }) == sizeof(struct ThenBox))
        : (sizeof(struct ElseBox { int value; }) == sizeof(struct ElseBox));
    struct ThenBox then_box = {3};
    struct ElseBox else_box = {5};
    total += then_box.value + else_box.value;

    total += 0 && (sizeof(struct AndBox { int value; }) == sizeof(struct AndBox));
    struct AndBox and_box = {7};
    total += and_box.value;

    total += 1 || (sizeof(union OrChoice { int value; char tag; }) == sizeof(union OrChoice));
    union OrChoice or_choice = {11};
    total += or_choice.value;

    return total;
}
