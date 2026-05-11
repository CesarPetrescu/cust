typedef int * const ConstIntSlot;

int main(void) {
    int left[1] = {1};
    int right[1] = {2};
    ConstIntSlot slot = left;
    slot = right;
    return slot[0];
}
