int sum_restrict(int * restrict values, int length) {
    int total = 0;
    for (int i = 0; i < length; i = i + 1) {
        total = total + values[i];
    }
    return total;
}

int bump_first(int * const restrict slot) {
    *slot = *slot + 4;
    return *slot;
}

struct Cursor {
    int * restrict p;
};

int main(void) {
    int values[3] = {2, 3, 5};
    int * restrict cursor = values + 1;
    int * volatile restrict noisy = values;
    struct Cursor holder = {values + 2};
    int total = sum_restrict(values, 3);
    total = total + cursor[1];
    total = total + noisy[0];
    total = total + *holder.p;
    total = total + bump_first(values);
    return total;
}
