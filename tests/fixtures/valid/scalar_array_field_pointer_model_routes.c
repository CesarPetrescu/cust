struct IntHolder {
    int items[4];
};

struct ConstIntHolder {
    const int items[4];
};

struct IntOuter {
    struct IntHolder holder;
};

union CharHolder {
    char items[4];
};

int main(void) {
    struct IntHolder left = {.items = {1, 2, 3, 4}};
    struct IntHolder right = {.items = {1, 2, 3, 4}};
    struct ConstIntHolder const_left = {.items = {1, 2, 3, 4}};
    struct IntOuter left_nested = {.holder = {.items = {1, 2, 3, 4}}};
    struct IntHolder *left_view = &left;

    union CharHolder char_left = {.items = {1, 2, 3, 4}};
    union CharHolder *char_left_view = &char_left;

    struct {
        int items[4];
    } anon_left = {.items = {1, 2, 3, 4}},
      *anon_left_view = &anon_left,
      anon_right = {.items = {1, 2, 3, 4}},
      *anon_right_view = &anon_right;

    union {
        char items[4];
    } anon_char_left = {.items = {1, 2, 3, 4}},
      *anon_char_left_view = &anon_char_left,
      anon_char_right = {.items = {1, 2, 3, 4}},
      *anon_char_right_view = &anon_char_right;

    int *i0 = left.items + 1;
    int *i1 = &left.items[2];
    int *i2 = left_view->items + 3;
    int *i3 = left_nested.holder.items + 1;
    int *i4 = 0 ? right.items + 3 : left.items;
    int marker = 0;
    int *i5 = (marker++, &right.items[1]);
    int *i6 = ((struct IntHolder){.items = {1, 2, 3, 4}}).items + 2;
    const int *i7 = const_left.items + 3;
    int *i8 = anon_left.items + 1;
    int *i9 = &anon_right_view->items[2];

    char *c0 = char_left.items + 1;
    char *c1 = &char_left.items[2];
    char *c2 = char_left_view->items + 3;
    char *c3 = anon_char_left.items + 1;
    char *c4 = &anon_char_right_view->items[2];
    char *c5 = ((union CharHolder){.items = {1, 2, 3, 4}}).items + 3;

    int differences = (&left.items[3] - left_view->items) +
                      (&anon_left.items[3] - anon_left_view->items) +
                      (&char_left.items[2] - char_left_view->items) +
                      (&anon_char_left.items[3] - anon_char_left_view->items);

    return *i0 + *i1 + *i2 + *i3 + *i4 + *i5 + *i6 + *i7 + *i8 + *i9 +
           *c0 + *c1 + *c2 + *c3 + *c4 + *c5 + marker + differences +
           anon_right.items[0] - anon_right.items[0] +
           anon_char_right.items[0] - anon_char_right.items[0];
}
