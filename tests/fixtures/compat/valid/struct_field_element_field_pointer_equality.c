struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[3];
    int bias;
};

struct Box {
    struct Line line;
    int tail;
};

int same_int_ptr(int *left, int *right) {
    return left == right;
}

int different_int_ptr(int *left, int *right) {
    return left != right;
}

int main(void) {
    struct Line line = {{{1, 2}, {3, 4}, {5, 6}}, 7};
    struct Point *mid = line.points + 1;
    struct Point *same_mid = line.points + 1;
    int *mid_x = &mid->x;
    int *same_mid_x = &same_mid->x;
    int *mid_y = &mid->y;

    struct Box box = {{{{8, 9}, {10, 11}, {12, 13}}, 14}, 15};
    struct Point *nested = box.line.points + 2;
    struct Point *same_nested = box.line.points + 2;
    struct Point *previous_nested = box.line.points + 1;
    int *nested_y = &nested->y;
    int *same_nested_y = &same_nested->y;
    int *previous_nested_y = &previous_nested->y;

    int score = 0;
    if (mid_x == same_mid_x) {
        score = score + 1;
    }
    if (same_mid_x == mid_x) {
        score = score + 2;
    }
    if (mid_x != mid_y) {
        score = score + 4;
    }
    if (same_int_ptr(nested_y, same_nested_y)) {
        score = score + 8;
    }
    if (different_int_ptr(nested_y, previous_nested_y)) {
        score = score + 16;
    }

    *mid_x = *mid_x + 20;
    *nested_y = *nested_y + 30;

    return score + line.points[1].x + box.line.points[2].y;
}
