struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

int sum_point(struct Point p) {
    return p.x + p.y;
}

int sum_union(union Number n) {
    return n.value;
}

int main(void) {
    struct Point source = {3, 4};
    struct Point target = {1, 2};
    struct Point copy = (target = source);

    source.x = 9;
    int struct_score = sum_point(target) + sum_point(copy);

    struct Point replacement = {5, 6};
    struct Point *slot = &target;
    struct Point deref_copy = (*slot = replacement);
    replacement.y = 20;
    struct_score = struct_score + sum_point(target) + sum_point(deref_copy);

    union Number one = {7};
    union Number two = {2};
    union Number selected = (two = one);
    one.value = 11;
    int union_score = sum_union(two) + sum_union(selected);

    union Number replacement_number = {8};
    union Number *number_slot = &two;
    union Number deref_number = (*number_slot = replacement_number);
    replacement_number.value = 12;
    union_score = union_score + sum_union(two) + sum_union(deref_number);

    return struct_score + union_score;
}
