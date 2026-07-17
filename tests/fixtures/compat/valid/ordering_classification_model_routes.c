struct Inner {
    int values[4];
};

struct Cursor {
    int *primary;
    struct Inner nested;
};

int *forward(int *value) {
    return value;
}

int main(void) {
    int left[4] = {4, 7, 9, 12};
    int right[4] = {4, 7, 9, 12};
    struct Cursor cursor = {left, {{0, 1, 2, 3}}};
    struct Cursor *view = &cursor;
    int marker = 0;
    int score = 0;

    if (view->nested.values[2] < +3) {
        score += 1;
    }
    if (forward(left) < cursor.primary + 1) {
        score += 2;
    }
    if (forward(left + 2) <= view->primary + 2) {
        score += 4;
    }
    if (cursor.primary + 3 > forward(left + 1)) {
        score += 8;
    }
    if ((1 ? left + 2 : right + 2) >= (marker++, forward(left + 2))) {
        score += 16;
    }
    if ((marker++, view->nested.values[1]) <= (int)1) {
        score += 32;
    }
    if ((marker++, forward(left + 1)) < forward(left + 3)) {
        score += 64;
    }

    return score + marker;
}
