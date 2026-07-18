struct Node {
    int * const cursor;
};

struct Box {
    struct Node nodes[1];
};

int main(void) {
    int values[2] = {3, 5};
    struct Box box = {{{values}}};
    box.nodes[0].cursor = values + 1;
    return 0;
}
