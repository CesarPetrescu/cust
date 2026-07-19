struct Point {
    int value;
};

struct Inner {
    struct Point points[2];
};

struct Item {
    struct Inner nested;
};

struct Wrapper {
    struct Item primary[1];
};

int inspect(struct Item items[]) {
    struct Point *first = &items[0].nested.points[0];
    struct Point *again = &0[items[0].nested.points];
    struct Point *second = &items[0].nested.points[1];
    int same = first == again;
    int distance = second - first;
    int ordered = first < second;
    first->value += 2;
    return same + distance + ordered + first->value + second->value;
}

int separate(struct Item left[], struct Item right[]) {
    struct Point *left_slot = &left[0].nested.points[0];
    struct Point *right_slot = &right[0].nested.points[0];
    return left_slot != right_slot;
}

int main(void) {
    struct Item root[1] = {{{{{3}, {7}}}}};
    struct Wrapper wrapper = {{{{{{5}, {9}}}}}};
    return inspect(root) + inspect(wrapper.primary)
        + separate(root, wrapper.primary);
}
