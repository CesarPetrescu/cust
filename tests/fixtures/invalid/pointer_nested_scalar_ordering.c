struct Inner {
    int values[2];
};

struct Cursor {
    struct Inner nested;
};

int main(void) {
    int values[2] = {1, 2};
    int *pointer = values;
    struct Cursor cursor = {{{3, 4}}};
    struct Cursor *view = &cursor;
    return pointer < view->nested.values[1];
}
