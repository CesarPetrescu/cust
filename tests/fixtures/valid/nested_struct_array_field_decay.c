struct Line {
    int values[3];
    char label[4];
};

struct Box {
    struct Line inner;
    int bias;
};

int sum_values(int values[]) {
    values[1] = values[1] + 5;
    return values[0] + values[1] + values[2];
}

int read_label(char text[]) {
    text[2] = 'Z';
    return text[0] + text[1] + text[2] + text[3];
}

int bump(int *slot) {
    *slot = *slot + 7;
    return *slot;
}

int main(void) {
    struct Box boxes[2] = {{{{1, 2, 3}, "ab"}, 0}, {{{4, 5, 6}, "cd"}, 0}};
    struct Box one = {{{7, 8, 9}, "ef"}, 10};
    struct Box *ptr = &one;

    int total = 0;
    total = total + sum_values(one.inner.values);       /* one: 7 + 13 + 9 = 29 */
    total = total + bump(&one.inner.values[2]);         /* one values[2] = 16 */
    total = total + sum_values(boxes[1].inner.values);  /* boxes[1]: 4 + 10 + 6 = 20 */
    total = total + bump(&boxes[0].inner.values[0]);    /* boxes[0] values[0] = 8 */
    total = total + sum_values(ptr->inner.values);      /* one: 7 + 18 + 16 = 41 */
    total = total + bump(&ptr->inner.values[0]);        /* one values[0] = 14 */

    total = total + read_label(one.inner.label);        /* 'e' + 'f' + 'Z' + 0 = 293 */
    total = total + read_label(boxes[0].inner.label);   /* 'a' + 'b' + 'Z' + 0 = 285 */
    total = total + read_label(ptr->inner.label);       /* 'e' + 'f' + 'Z' + 0 = 293 */

    return (total + one.inner.values[0] + one.inner.values[1] + one.inner.values[2]
        + boxes[0].inner.values[0] + boxes[1].inner.values[1]) % 256;
}
