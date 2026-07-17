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

int *choose(int *value, int enabled) {
    if (enabled) {
        return value;
    }
    return (int *)0;
}

int main(void) {
    int left[4] = {4, 7, 9, 12};
    int right[4] = {4, 7, 9, 12};
    struct Cursor cursor = {left, {{0, 1, 2, 3}}};
    struct Cursor *view = &cursor;
    int marker = 0;
    int loops = 0;
    int score = 0;

    if (view->nested.values[2]) {
        score += 1;
    }
    if (!((int *)0)) {
        score += 2;
    }
    if (forward(left + 1) && (marker++, 1)) {
        score += 4;
    }
    if ((int *)0 || (marker++, 1)) {
        score += 8;
    }
    score += choose(1 ? cursor.primary + 2 : right, view->nested.values[1]) ? 16 : 0;
    score += (marker++, (int *)0) ? 32 : 0;
    score += ((left + 3) - left) ? 64 : 0;
    while (choose(left + loops, view->nested.values[1]) && loops < 1) {
        loops++;
    }
    if ((int *)choose(left + 1, view->nested.values[2])) {
        score += 128;
    }

    return score + marker + loops;
}
