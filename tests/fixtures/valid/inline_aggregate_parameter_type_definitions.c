int sum_pair(struct Pair { int x; int y; } point) {
    struct Pair copy = point;
    copy.x += 3;
    return copy.x + copy.y + sizeof(struct Pair);
}

int inspect_union(union Number { int value; char tag; } number) {
    union Number copy = number;
    copy.value += 5;
    return copy.value + sizeof(union Number);
}

int main(void) {
    return 19;
}
