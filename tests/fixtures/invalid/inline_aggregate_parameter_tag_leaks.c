int sum_pair(struct Pair { int x; int y; } point) {
    struct Pair copy = point;
    return copy.x + copy.y;
}

int main(void) {
    struct Pair leaked = {1, 2};
    return leaked.x + leaked.y;
}
