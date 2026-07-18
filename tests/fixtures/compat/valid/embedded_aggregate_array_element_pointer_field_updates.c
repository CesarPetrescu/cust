struct Node {
    int *cursor;
};

struct Layer {
    struct Node nodes[1];
};

struct Box {
    struct Layer layer;
};

struct Index {
    int value;
};

int main(void) {
    int values[4] = {2, 4, 6, 8};
    struct Node loose[1] = {{values}};
    struct Index index = {0};
    struct Box box = {{{{values}}}};
    int ok = 0;

    int *direct_result = (box.layer.nodes[0].cursor = values + 1);
    int *reverse_result = (index.value[loose].cursor = values + 2);
    int *compound_result = (box.layer.nodes[0].cursor += 1);
    int *post_result = box.layer.nodes[0].cursor++;
    int *pre_result = --box.layer.nodes[0].cursor;

    ok += *direct_result == 4;
    ok += *reverse_result == 6;
    ok += *index.value[loose].cursor == 6;
    ok += *compound_result == 6;
    ok += *post_result == 6;
    ok += *pre_result == 6;
    ok += *box.layer.nodes[0].cursor == 6;
    ok += sizeof(direct_result) == sizeof(int *);
    ok += sizeof(box.layer.nodes[0].cursor) == sizeof(int *);

    return ok;
}
