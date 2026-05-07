struct Node {
    int value;
    struct Node *next;
    int *external;
};

int read_next(struct Node node) {
    node.next->value += 1;
    return node.next->value;
}

int main(void) {
    int bonus = 5;
    struct Node tail = {7, 0, &bonus};
    struct Node head = {3, &tail, 0};

    int before = head.next->value;
    head.next->value = head.next->value + *tail.external;
    int via_copy = read_next(head);

    int other_value = 2;
    struct Node other = {2, 0, 0};
    head.next = &other;
    head.external = &other_value;
    *head.external += 4;

    return before + tail.value + via_copy + head.next->value + other_value;
}
