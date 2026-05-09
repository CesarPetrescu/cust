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
    const struct Box box = {{{1, 2}}};
    return mutate(box.inner.values);
}
