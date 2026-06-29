int take(int left, int middle, int right) {
    return left + middle + right;
}

int main(void) {
    int total = take(
        sizeof(struct ArgBox { int value; }) == sizeof(struct ArgBox),
        sizeof(union ArgChoice { int value; char tag; }) == sizeof(union ArgChoice),
        ((struct LitBox { int value; }){5}).value
    );

    struct ArgBox box = {7};
    union ArgChoice choice = {11};
    struct LitBox lit = {13};

    return total + box.value + choice.value + lit.value;
}
