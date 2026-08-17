struct Item {
    double values[2];
};

struct Holder {
    struct Item *items;
};

int main(void) {
    struct Item direct = {{1.0, 2.0}};
    struct Item indirect = {{3.0, 4.0}};
    struct Holder holder = {&direct};
    struct Holder holders[1] = {{&indirect}};
    const struct Holder const_holder = {&direct};
    const struct Holder *slot = &const_holder;
    int direct_index = 0;
    int wrapped_index = 0;
    double first = holder.items[direct_index++].values[0];
    double second = holders[0].items[wrapped_index++].values[1];

    if (first != 1.0 || second != 4.0 || direct_index != 1 || wrapped_index != 1) {
        return 1;
    }
    if (!_Generic(holder.items[0].values[0], double: 1, default: 0)
        || !_Generic(slot->items[0], struct Item: 1, default: 0)) {
        return 2;
    }
    if (sizeof(holder.items[direct_index].values[0]) != sizeof(double)
        || sizeof(holders[0].items[wrapped_index].values[1]) != sizeof(double)
        || sizeof(slot->items[0].values[0]) != sizeof(double)) {
        return 3;
    }
    if (direct_index != 1 || wrapped_index != 1) {
        return 4;
    }

    slot->items[0].values[0] = 5.0;
    holders[0].items[0].values[1] += 2.0;
    return direct.values[0] == 5.0 && indirect.values[1] == 6.0 ? 0 : 5;
}
