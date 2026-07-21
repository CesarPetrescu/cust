struct Point {
    int value;
};

struct Inner {
    int values[2];
    struct Point points[2];
};

struct Item {
    struct Inner nested;
};

struct Item *forward_items(struct Item *items) {
    return items;
}

struct Item *forward_items_twice(struct Item *items) {
    return forward_items(items);
}

const struct Item *promote_items(struct Item *items) {
    return items;
}

const struct Item *promote_items_twice(struct Item *items) {
    return promote_items(forward_items(items));
}

const struct Item *forward_const_items(const struct Item *items) {
    return items;
}

const struct Item *forward_const_items_twice(const struct Item *items) {
    return forward_const_items(items);
}

int read_promoted(const struct Item items[]) {
    const struct Item *original = items;
    const int *scalar = &items[0].nested.values[1];
    const struct Point *point = &items[0].nested.points[1];
    int score = (items[-1].nested.values[0] == 9)
        + (*scalar == 11)
        + (point->value == 13)
        + (scalar == &items[0].nested.values[1])
        + (point == &items[0].nested.points[1])
        + (original == items);
    const struct Item *fallback = items - 1;
    items = fallback;
    scalar = &fallback[0].nested.values[0];
    point = &fallback[0].nested.points[0];
    return score + (items == fallback)
        + (scalar == &fallback[0].nested.values[0])
        + (point == &fallback[0].nested.points[0]);
}

int main(void) {
    int a_selected = 0;
    int a_unselected = 0;
    int b_selected = 0;
    int b_unselected = 0;
    int c_selected = 0;
    int c_comma = 0;
    int d_selected = 0;
    int d_unselected = 0;

    int score = read_promoted(
        forward_const_items(
            1
                ? promote_items(
                      (struct Item[3]){
                          {.nested = {.values = {9}}},
                          {.nested = {.values = {++a_selected, 11}, .points = {[1] = {13}}}},
                          {}}
                  )
                : (++a_unselected, promote_items((struct Item[3]){{}, {}, {}}))
        )
        + 1
    );
    score += read_promoted(
        forward_const_items_twice(
            1
            + (0
                   ? (++b_unselected, promote_items_twice((struct Item[3]){{}, {}, {}}))
                   : promote_items_twice(
                         (struct Item[3]){
                             {.nested = {.values = {9}}},
                             {.nested = {.values = {++b_selected, 11}, .points = {[1] = {13}}}},
                             {}}
                     ))
        )
    );
    score += read_promoted(
        promote_items(
            (++c_comma,
             forward_items(
                 (struct Item[3]){
                     {.nested = {.values = {9}}},
                     {.nested = {.values = {++c_selected, 11}, .points = {[1] = {13}}}},
                     {}}
             ))
        )
        + 1
    );
    score += read_promoted(
        promote_items_twice(
            &(1
                  ? forward_items_twice(
                        (struct Item[3]){
                            {.nested = {.values = {9}}},
                            {.nested = {.values = {++d_selected, 11}, .points = {[1] = {13}}}},
                            {}}
                    )
                  : (++d_unselected, forward_items_twice((struct Item[3]){{}, {}, {}})))[1]
        )
    );

    int markers = a_selected == 1 && a_unselected == 0
        && b_selected == 1 && b_unselected == 0
        && c_selected == 1 && c_comma == 1
        && d_selected == 1 && d_unselected == 0;
    return score + markers;
}
