struct Holder {
    const int *view;
    int *slot;
};

int main(void) {
    const int value = 5;
    struct Holder holder = {&value, 0};

    holder.slot = holder.view;
    return 0;
}
