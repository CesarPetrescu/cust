struct Inner {
    int values[4];
};

struct Cursor {
    int *primary;
    int *secondary;
    struct Inner nested;
};

int *forward(int *value) {
    return value;
}

int main(void) {
    int left[4] = {4, 7, 9, 12};
    int right[4] = {4, 7, 9, 12};
    struct Cursor cursor = {left, right, {{0, 1, 2, 3}}};
    struct Cursor *view = &cursor;
    int marker = 0;
    int score = 0;

    if ((marker++, view->nested.values[2]) == +2) {
        score += 1;
    }
    if ((0 ? 3 : 0) == (left - left)) {
        score += 2;
    }
    if (left == &left[0]) {
        score += 4;
    }
    if (forward(left + 1) == cursor.primary + 1) {
        score += 8;
    }
    if (view->secondary != left) {
        score += 16;
    }
    if ((1 ? left + 2 : right + 2) == &left[2]) {
        score += 32;
    }
    if ((marker++, left + 3) == &left[3]) {
        score += 64;
    }
    if (forward(left) == +0) {
        return 250;
    }
    if ((int)0 == forward(left)) {
        return 251;
    }

    return score + marker;
}
