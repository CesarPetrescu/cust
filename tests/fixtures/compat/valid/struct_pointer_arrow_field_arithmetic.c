struct Cursor {
    int *p;
};

int main(void) {
    int values[5] = {3, 4, 6, 9, 12};
    struct Cursor cursor = {values};
    struct Cursor *slot = &cursor;

    int first = *slot->p;
    slot->p += 3;
    int fourth = *slot->p;
    slot->p--;
    int third = *slot->p;
    ++slot->p;
    int fourth_again = *slot->p;
    slot->p = slot->p - 2;
    int second = *slot->p;

    return first + fourth * 2 + third * 3 + fourth_again * 5 + second * 7;
}
