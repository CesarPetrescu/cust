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

typedef const struct Item ConstItems[3];

struct Item *forward_items(struct Item *items) {
    return items;
}

struct Item *forward_items_twice(struct Item *items) {
    return forward_items(items);
}

const struct Item *forward_const_items(const struct Item *items) {
    return items;
}

const struct Item *forward_const_items_twice(const struct Item *items) {
    return forward_const_items(items);
}

int mutate_slots(int *scalar, struct Point *point, int *fallback, struct Point *point_fallback) {
    *scalar += 2;
    point->value += 3;
    int score = (*scalar == 7) + (point->value == 10);
    scalar = fallback;
    point = point_fallback;
    return score + (scalar == fallback) + (point == point_fallback);
}

int probe(struct Item items[]) {
    int *scalar = &items[0].nested.values[1];
    struct Point *point = &items[0].nested.points[1];
    int score = mutate_slots(scalar, point, scalar, point);
    score += scalar == &items[0].nested.values[1];
    score += point == &items[0].nested.points[1];
    score += items[-1].nested.values[0] == 9;
    score += *scalar == 7;
    score += point->value == 10;
    return score;
}

int read_const(const struct Item items[]) {
    return (items[-1].nested.values[0] == 9)
        + (items[0].nested.values[1] == 11)
        + (items[0].nested.points[1].value == 13);
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
    int e_selected = 0;
    int e_unselected = 0;
    int f_selected = 0;
    int f_comma = 0;
    int g_selected = 0;
    int g_unselected = 0;
    int h_selected = 0;
    int h_comma = 0;

    int score = probe(
        forward_items(
            1
                ? (struct Item[3]){
                      {.nested = {.values = {9}}},
                      {.nested = {.values = {++a_selected, 5}, .points = {[1] = {7}}}},
                      {}}
                : (struct Item[3]){{}, {.nested = {.values = {++a_unselected}}}, {}}
        )
        + 1
    );
    score += probe(
        forward_items_twice(
            1
            + (0
                   ? (struct Item[3]){{}, {.nested = {.values = {++b_unselected}}}, {}}
                   : (struct Item[3]){
                         {.nested = {.values = {9}}},
                         {.nested = {.values = {++b_selected, 5}, .points = {[1] = {7}}}},
                         {}})
        )
    );
    score += probe(
        &forward_items_twice(
            (++c_comma,
             (struct Item[3]){
                 {.nested = {.values = {9}}},
                 {.nested = {.values = {++c_selected, 5}, .points = {[1] = {7}}}},
                 {}})
        )[1]
    );
    score += probe(
        forward_items(
            &((1
                   ? (struct Item[3]){
                         {.nested = {.values = {9}}},
                         {.nested = {.values = {++d_selected, 5}, .points = {[1] = {7}}}},
                         {}}
                   : (struct Item[3]){{}, {.nested = {.values = {++d_unselected}}}, {}})[1])
        )
    );

    score += read_const(
        forward_const_items(
            1
                ? (ConstItems){
                      {.nested = {.values = {9}}},
                      {.nested = {.values = {++e_selected, 11}, .points = {[1] = {13}}}},
                      {}}
                : (ConstItems){{}, {.nested = {.values = {++e_unselected}}}, {}}
        )
        + 1
    );
    score += read_const(
        forward_const_items_twice(
            1
            + (++f_comma,
               (ConstItems){
                   {.nested = {.values = {9}}},
                   {.nested = {.values = {++f_selected, 11}, .points = {[1] = {13}}}},
                   {}})
        )
    );
    score += read_const(
        &forward_const_items_twice(
            0
                ? (ConstItems){{}, {.nested = {.values = {++g_unselected}}}, {}}
                : (ConstItems){
                      {.nested = {.values = {9}}},
                      {.nested = {.values = {++g_selected, 11}, .points = {[1] = {13}}}},
                      {}}
        )[1]
    );
    score += read_const(
        forward_const_items(
            &(++h_comma,
              (ConstItems){
                  {.nested = {.values = {9}}},
                  {.nested = {.values = {++h_selected, 11}, .points = {[1] = {13}}}},
                  {}})[1]
        )
    );

    int markers = a_selected == 1 && a_unselected == 0
        && b_selected == 1 && b_unselected == 0
        && c_selected == 1 && c_comma == 1
        && d_selected == 1 && d_unselected == 0
        && e_selected == 1 && e_unselected == 0
        && f_selected == 1 && f_comma == 1
        && g_selected == 1 && g_unselected == 0
        && h_selected == 1 && h_comma == 1;
    return score + markers;
}
