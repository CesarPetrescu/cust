typedef int *IntPtr;
typedef const int *ConstIntPtr;

int read_const(const int *p) {
    if (p == (const int *)0) {
        return 0;
    }
    return *p;
}

int main(void) {
    int values[3] = {4, 6, 8};
    int *null_int = (int *)0;
    const int *const_null = (const int *)0;
    IntPtr cursor = (IntPtr)(values + 1);
    ConstIntPtr view = (ConstIntPtr)cursor;
    int *same = (int *)cursor;
    int size_score = (sizeof(*(char *)0) == sizeof(char)) + (sizeof(*(const int *)0) == sizeof(int));

    if (null_int != 0 || const_null != (const int *)0) {
        return 1;
    }

    same = (int *)(cursor + 1);
    return *cursor + *same + read_const(view) + size_score;
}
