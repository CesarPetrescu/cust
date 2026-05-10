struct Cursor {
    int *p;
};

int read_previous(int *p) {
    return p[-1];
}

int main(void) {
    int values[4] = {3, 5, 7, 11};
    int total = 0;

    total = total + (((struct Cursor){values}).p = values + 2)[-1];
    total = total + read_previous(((struct Cursor){values + 1}).p += 2);
    total = total + *(--((struct Cursor){values + 3}).p);
    total = total + *((struct Cursor){values + 1}).p++;

    return total;
}
