int main(void) {
    int i = 0;
    int total = 0;
    for (const struct { int limit; int step; } cfg = {3, 2}, *view = &cfg; i < view->limit; i++) {
        total += view->step + i;
    }

    int unions = 0;
    for (volatile union { int value; char tag; } number = {5}; unions < number.value; unions += 2) {
        total += unions;
    }

    return total + i + unions;
}
