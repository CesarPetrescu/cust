int main(void) {
    int total = 0;

    total += ((struct ExprPoint { int x; int y; }){3, 4}).x;
    struct ExprPoint point = {5, 6};
    total += point.y;

    total += ((union ExprNumber { int value; char tag; }){9}).value;
    union ExprNumber number = {10};
    total += number.value;

    total += sizeof(struct SizeBox { char tag; int value; }) == sizeof(struct SizeBox);
    struct SizeBox box = {'A', 7};
    total += box.value;

    return total;
}
