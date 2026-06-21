typedef int *IntPtr, * const ConstIntSlot;

int main(void) {
    int values[2] = {1, 2};
    ConstIntSlot slot = values;
    slot = values + 1;
    return *slot;
}
