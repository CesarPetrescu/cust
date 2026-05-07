int main(void) {
    typedef struct Pair {
        int x;
        int y;
    } Pair;
    Pair p = {7, 8};
    return p.x + p.y;
}
