int main(void) {
    _Static_assert(sizeof(struct AssertBox { int value; char tag; }) == sizeof(struct AssertBox), "struct tag visible in assertion");
    struct AssertBox box = {7, 3};

    _Static_assert(sizeof(union AssertChoice { int value; char tag; }) == sizeof(union AssertChoice), "union tag visible in assertion");
    union AssertChoice choice = {11};

    return box.value + box.tag + choice.value;
}
