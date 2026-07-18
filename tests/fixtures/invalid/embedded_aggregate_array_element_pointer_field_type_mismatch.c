struct Node {
    int *cursor;
};

struct Box {
    struct Node nodes[1];
};

int main(void) {
    char bytes[1] = {'a'};
    struct Box box = {{{0}}};
    box.nodes[0].cursor = bytes;
    return 0;
}
