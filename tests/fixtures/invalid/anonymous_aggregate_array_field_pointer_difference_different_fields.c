struct Box {
    struct { int x; int y; } items[2];
};

int main(void) {
    struct Box left = {{{1, 2}, {3, 4}}};
    struct Box right = {{{5, 6}, {7, 8}}};
    return right.items - left.items;
}
