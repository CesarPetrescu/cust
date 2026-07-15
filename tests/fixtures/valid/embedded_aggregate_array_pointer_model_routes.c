struct Point {
    int value;
};

union Number {
    int value;
    char tag;
};

struct PointHolder {
    struct Point items[4];
};

struct ConstPointHolder {
    const struct Point items[4];
};

struct PointOuter {
    struct PointHolder holder;
};

union NumberHolder {
    union Number items[4];
};

int main(void) {
    struct PointHolder p_left = {{{1}, {2}, {3}, {4}}};
    struct PointHolder p_right = {{{5}, {6}, {7}, {8}}};
    struct ConstPointHolder const_p = {{{9}, {10}, {11}, {12}}};
    struct PointOuter scene = {{{{13}, {14}, {15}, {16}}}};
    struct PointHolder *p_view = &p_left;

    union NumberHolder n_left = {{{17}, {18}, {19}, {20}}};
    union NumberHolder n_right = {{{21}, {22}, {23}, {24}}};
    union NumberHolder *n_view = &n_left;

    struct {
        struct Point items[4];
    } anon_p_left = {{{25}, {26}, {27}, {28}}},
      *anon_p_left_view = &anon_p_left,
      anon_p_right = {{{29}, {30}, {31}, {32}}},
      *anon_p_right_view = &anon_p_right;

    union {
        union Number items[4];
    } anon_n_left = {{{33}, {34}, {35}, {36}}},
      *anon_n_left_view = &anon_n_left,
      anon_n_right = {{{37}, {38}, {39}, {40}}},
      *anon_n_right_view = &anon_n_right;

    struct Point *p0 = p_left.items + 1;
    struct Point *p1 = &p_left.items[2];
    struct Point *p2 = p_view->items + 3;
    struct Point *p3 = scene.holder.items + 1;
    struct Point *p4 = 0 ? p_right.items + 3 : p_left.items;
    int marker = 0;
    struct Point *p5 = (marker++, &p_right.items[1]);
    struct Point *p6 = ((struct PointHolder){{{5}, {6}, {7}, {8}}}).items + 2;
    const struct Point *p7 = const_p.items + 3;

    union Number *n0 = n_left.items + 1;
    union Number *n1 = &n_left.items[2];
    union Number *n2 = n_view->items + 3;
    struct Point *p8 = anon_p_left.items + 1;
    struct Point *p9 = &anon_p_right_view->items[2];
    union Number *n3 = anon_n_left.items + 1;
    union Number *n4 = &anon_n_right_view->items[3];

    int differences = (&p_left.items[3] - p_view->items) +
                      (&anon_p_left.items[3] - anon_p_left_view->items) +
                      (&n_left.items[2] - n_view->items);

    return p0->value + p1->value + p2->value + p3->value + p4->value +
           p5->value + p6->value + p7->value + n0->value + n1->value +
           n2->value + p8->value + p9->value + n3->value + n4->value +
           marker + differences + anon_p_right.items[0].value -
           anon_p_right.items[0].value + anon_n_right.items[0].value -
           anon_n_right.items[0].value + n_right.items[0].value -
           n_right.items[0].value + anon_n_left_view->items[0].value -
           anon_n_left_view->items[0].value;
}
