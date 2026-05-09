struct Line {
    int values[2];
};

struct Box {
    struct Line inner;
};

int mutate(int values[]) {
    values[0] = 5;
    return values[0];
}

int main(void) {
    const struct Box boxes[1] = {{{{1, 2}}}};
    return mutate(boxes[0].inner.values);
}
