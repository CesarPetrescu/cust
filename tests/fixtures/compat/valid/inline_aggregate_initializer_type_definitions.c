int main(void) {
    struct Pair { int x; int y; } pair = {
        sizeof(struct InitBox { int value; }) == sizeof(struct InitBox),
        ((struct LitBox { int value; }){6}).value
    };
    struct InitBox init = {5};
    struct LitBox lit = {7};

    struct Holder { int values[3]; int marker; } holder = {
        .values[sizeof(struct IndexBox { int value; }) == sizeof(struct IndexBox)] = 9,
        .marker = sizeof(union MarkChoice { int value; char tag; }) == sizeof(union MarkChoice)
    };
    struct IndexBox index = {3};
    union MarkChoice mark = {4};

    return pair.x + pair.y + init.value + lit.value + holder.values[1] + holder.marker + index.value + mark.value;
}
