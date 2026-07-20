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

int a_selected;
int a_unselected;
int b_selected;
int b_unselected;
int c_selected;
int c_unselected;
int c_comma;
int d_selected;
int d_unselected;
int e_selected;
int e_unselected;
int e_comma;
int f_selected;
int f_unselected;

int *forward_int(int *value) {
    return value;
}

struct Point *forward_point(struct Point *value) {
    return value;
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
    int *scalar = forward_int(&items[0].nested.values[1]);
    struct Point *point = forward_point(&items[0].nested.points[1]);
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
    int score = probe(
        (1
             ? (struct Item[3]){
                   {.nested = {.values = {9}}},
                   {.nested = {.values = {++a_selected, 5}, .points = {[1] = {7}}}},
                   {}}
             : (struct Item[3]){{}, {.nested = {.values = {++a_unselected}}}, {}})
        + 1
    );
    score += probe(
        1
        + (0
               ? (struct Item[3]){{}, {.nested = {.values = {++b_unselected}}}, {}}
               : (struct Item[3]){
                     {.nested = {.values = {9}}},
                     {.nested = {.values = {++b_selected, 5}, .points = {[1] = {7}}}},
                     {}})
    );
    score += probe(
        &((++c_comma,
           (struct Item[3]){
               {.nested = {.values = {9}}},
               {.nested = {.values = {++c_selected, 5}, .points = {[1] = {7}}}},
               {}})[1])
    );

    score += read_const(
        (1
             ? (ConstItems){
                   {.nested = {.values = {9}}},
                   {.nested = {.values = {++d_selected, 11}, .points = {[1] = {13}}}},
                   {}}
             : (ConstItems){{}, {.nested = {.values = {++d_unselected}}}, {}})
        + 1
    );
    score += read_const(
        1
        + (++e_comma,
           (ConstItems){
               {.nested = {.values = {9}}},
               {.nested = {.values = {++e_selected, 11}, .points = {[1] = {13}}}},
               {}})
    );
    score += read_const(
        &((0
               ? (ConstItems){{}, {.nested = {.values = {++f_unselected}}}, {}}
               : (ConstItems){
                     {.nested = {.values = {9}}},
                     {.nested = {.values = {++f_selected, 11}, .points = {[1] = {13}}}},
                     {}})[1])
    );

    int markers = a_selected == 1 && a_unselected == 0
        && b_selected == 1 && b_unselected == 0
        && c_selected == 1 && c_unselected == 0 && c_comma == 1
        && d_selected == 1 && d_unselected == 0
        && e_selected == 1 && e_unselected == 0 && e_comma == 1
        && f_selected == 1 && f_unselected == 0;
    return score + markers;
}
