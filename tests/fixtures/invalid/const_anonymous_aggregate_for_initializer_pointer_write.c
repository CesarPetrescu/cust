int main(void) {
    int i = 0;
    for (const struct { int x; } point = {1}, *slot = &point; i < slot->x; i++) {
        slot->x = 9;
    }
    return i;
}
