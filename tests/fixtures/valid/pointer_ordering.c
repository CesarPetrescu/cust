struct Point {
    int x;
};

int scalar_order(int *left, int *middle, int *right) {
    int *same = middle;
    int score = 0;
    if (left < middle) {
        score += 1;
    }
    if (middle <= same) {
        score += 2;
    }
    if (right > middle) {
        score += 4;
    }
    if (middle >= left) {
        score += 8;
    }
    return score;
}

int aggregate_order(struct Point *first, struct Point *second) {
    struct Point *same = first;
    if (first < second && second > first && first <= same && second >= first) {
        return second->x;
    }
    return 0;
}

int main(void) {
    int values[4] = {2, 4, 6, 8};
    struct Point points[2] = {{3}, {5}};
    char *text = "abc";
    char *same = text + 1;
    int string_score = (text < text + 2) + ((text + 1) <= same) * 2 + ((text + 2) > text) * 3;
    return scalar_order(values, values + 1, &values[3]) + aggregate_order(points, points + 1) + string_score;
}
