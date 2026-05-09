int mutate(int *slot) {
    int *same = &*slot;
    *same = *same + 3;
    return *slot;
}

int main(void) {
    int value = 4;
    int values[3] = {1, 2, 3};
    int *p = &value;
    int *q = &*p;
    int *r = &*(values + 1);
    int *null_ptr = 0;
    int *still_null = &*null_ptr;
    *q = *q + 5;
    *r = *r + 7;
    return value + values[1] + mutate(&value) + (still_null == 0);
}
