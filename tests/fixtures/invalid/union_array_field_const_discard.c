union Number {
    int value;
    char tag;
};

struct Bag {
    union Number numbers[2];
};

void mutate(union Number numbers[]) {
    numbers[0].value = 7;
}

int main(void) {
    const struct Bag bag = {{{1}, {2}}};
    mutate(bag.numbers);
    return 0;
}
