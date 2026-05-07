union Node {
    int value;
    union Node **next;
};

int main(void) {
    union Node node = {1};
    return node.value;
}
