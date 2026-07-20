struct Point {
    int value;
};

struct Item {
    int values[3];
    struct Point points[3];
};

typedef const struct Item ConstItems[2];

int a_selected;
int a_unselected;
int b_selected;
int b_unselected;
int c_selected;
int c_unselected;
int c_comma;
int left_selected;
int left_unselected;
int right_selected;
int right_unselected;
int right_comma;
int d_selected;
int d_unselected;
int e_selected;
int e_unselected;
int e_comma;

int *forward_int(int *value) {
    return value;
}

int *forward_int_twice(int *value) {
    return forward_int(value);
}

const int *forward_const_int(const int *value) {
    return value;
}

int mutate(int *first, int *second, const int *reader, int *fallback) {
    *first = 20;
    int before = *reader;
    *second += 3;
    int after = *reader;
    first = fallback;
    second = fallback;
    reader = fallback;
    return (before == 20) + (after == 23)
        + (first == fallback) + (second == fallback) + (reader == fallback);
}

int probe(struct Item items[]) {
    int *first = forward_int_twice(&items[0].values[1]);
    int *second = forward_int(&1[items[0].values]);
    const int *reader = forward_const_int(&items[0].values[1]);
    *first = 4;
    int score = mutate(first, second, reader, first);
    score += first == &items[0].values[1];
    score += second == &1[items[0].values];
    score += reader == &items[0].values[1];
    score += *first == 23;
    score += *second == 23;
    score += *reader == 23;
    return score;
}

int probe_separate(struct Item first_items[], struct Item second_items[]) {
    int *first = forward_int_twice(&first_items[0].values[0]);
    int *second = forward_int(&0[second_items[0].values]);
    const int *reader = forward_const_int(&second_items[0].values[0]);
    *first = 4;
    *second = 8;
    int score = mutate(first, second, reader, first);
    score += first == &first_items[0].values[0];
    score += second == &0[second_items[0].values];
    score += reader == &second_items[0].values[0];
    score += *first == 20;
    score += *second == 11;
    score += *reader == 11;
    return score;
}

int read_const(const struct Item items[]) {
    return items[0].values[0] + items[1].points[2].value;
}

int main(void) {
    int score = probe(
        1
            ? (struct Item[2]){{.values = {++a_selected}}, {}}
            : (struct Item[2]){{.values = {++a_unselected}}, {}}
    );
    score += probe(
        0
            ? (struct Item[2]){{.values = {++b_unselected}}, {}}
            : (struct Item[2]){{.values = {++b_selected}}, {}}
    );
    score += probe((++c_comma, (struct Item[2]){{.values = {++c_selected}}, {}}));
    score += probe_separate(
        1
            ? (struct Item[2]){{.values = {++left_selected}}, {}}
            : (struct Item[2]){{.values = {++left_unselected}}, {}},
        (++right_comma, (struct Item[2]){{.values = {++right_selected}}, {}})
    );
    score += read_const(
        1
            ? (ConstItems){{.values = {7 + ++d_selected}}, {.points = {[2] = {9}}}}
            : (ConstItems){{.values = {++d_unselected}}, {}}
    );
    score += read_const(
        (++e_comma, (ConstItems){{.values = {3 + ++e_selected}}, {.points = {[2] = {5}}}})
    );
    int markers = a_selected == 1 && a_unselected == 0
        && b_selected == 1 && b_unselected == 0
        && c_selected == 1 && c_unselected == 0 && c_comma == 1
        && left_selected == 1 && left_unselected == 0
        && right_selected == 1 && right_unselected == 0 && right_comma == 1
        && d_selected == 1 && d_unselected == 0
        && e_selected == 1 && e_unselected == 0 && e_comma == 1;
    return score + markers;
}
