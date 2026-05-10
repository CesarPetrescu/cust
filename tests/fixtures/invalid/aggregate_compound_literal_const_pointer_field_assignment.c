struct Cursor {
    int * const p;
};

int main(void) {
    int values[2] = {1, 2};
    return *(((struct Cursor){values}).p = values + 1);
}
