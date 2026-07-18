struct Node {
    int *cursor;
};

struct Box {
    struct Node nodes[1];
};

int main(void) {
    const int values[1] = {3};
    struct Box box = {{{0}}};
    box.nodes[0].cursor = values;
    return 0;
}
