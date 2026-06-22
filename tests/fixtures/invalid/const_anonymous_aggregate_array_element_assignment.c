int main(void) {
    const struct { int x; int y; } points[2] = {{1, 2}, {3, 4}}, *slot = points + 1;
    slot->x = 9;
    return slot->x;
}
