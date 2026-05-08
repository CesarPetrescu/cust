struct Cursor {
    int *p;
};

int main(void) {
    int values[4] = {2, 5, 7, 11};
    struct Cursor cursor = {values};

    int first = *cursor.p;
    cursor.p += 2;
    int middle = *cursor.p;
    cursor.p--;
    int before = *cursor.p;
    ++cursor.p;
    int after = *cursor.p;
    cursor.p = cursor.p - 1;
    int assigned = *cursor.p;

    return first + middle * 2 + before * 3 + after * 5 + assigned * 7;
}
