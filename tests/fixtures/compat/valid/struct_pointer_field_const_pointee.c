struct Holder {
    const int *view;
    int *slot;
};

int main(void) {
    int first = 5;
    int second = 7;
    struct Holder holder = {&first, &first};

    int before = *holder.view;
    holder.view = &second;
    *holder.slot = 6;

    return before + *holder.view + first;
}
