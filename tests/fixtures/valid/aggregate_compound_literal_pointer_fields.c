struct Cursor {
    int *p;
};

int pointed(int *p) {
    return *p;
}

int main(void) {
    int values[3] = {4, 5, 6};
    int *middle = ((struct Cursor){values + 1}).p;
    int total = pointed(middle);
    total = total + *(((struct Cursor){values}).p + 2);
    if (((struct Cursor){values}).p == values) {
        total = total + 1;
    }
    return total;
}
