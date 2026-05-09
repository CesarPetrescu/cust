struct Cursor {
    const int *p;
};

int main(void) {
    const int value = 7;
    int *mutable = ((struct Cursor){&value}).p;
    return *mutable;
}
