const struct { int x; int y; } origin = {2, 3};

int main(void) {
    struct { int x; int y; } point = {4, 5}, copy = point, *slot = &point;
    union { int value; char tag; } number = {6}, *number_slot = &number;
    slot->x = slot->x + copy.y;
    number_slot->value = 7;
    return origin.x + origin.y + point.x + point.y + number.value;
}
