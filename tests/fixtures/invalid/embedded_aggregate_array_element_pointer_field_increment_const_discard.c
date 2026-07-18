struct Node {
    const int *cursor;
};

struct Box {
    struct Node nodes[1];
};

int main(void) {
    int values[2] = {3, 5};
    struct Box box = {{{values}}};
    int *mutable_view = box.nodes[0].cursor++;
    return *mutable_view;
}
