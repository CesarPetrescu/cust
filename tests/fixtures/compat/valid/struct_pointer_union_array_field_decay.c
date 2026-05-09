union Number {
    int value;
    char tag;
};

struct Bag {
    union Number numbers[3];
    int bias;
};

int adjust_numbers(union Number numbers[]) {
    numbers[0].tag += 2;
    numbers[2].value = numbers[0].value + numbers[1].value;
    return numbers[0].tag + numbers[2].value;
}

int bump_number(union Number *number) {
    number->value += 4;
    return number->tag;
}

int use_bag(struct Bag *bag) {
    int total = adjust_numbers(bag->numbers);    /* numbers[0] = 3, numbers[2] = 6, returns 9 */
    union Number *last = &bag->numbers[2];
    total += bump_number(last);                  /* numbers[2] = 10, returns 10 */
    last = bag->numbers + 1;
    total += last->value;                        /* 3 */
    return total + bag->numbers[2].tag;
}

int main(void) {
    struct Bag bag = {{{1}, {3}, {5}}, 7};
    return (use_bag(&bag) + bag.numbers[0].tag + bag.bias) % 256;
}
