union Number {
    int value;
    char tag;
};

struct Bag {
    union Number numbers[3];
    int bias;
};

int adjust_numbers(union Number numbers[]) {
    numbers[1].value += 5;
    numbers[2].tag = numbers[0].tag + numbers[1].tag;
    return numbers[1].value + numbers[2].tag;
}

int bump_number(union Number *number) {
    number->value += 7;
    return number->value;
}

int main(void) {
    struct Bag bag = {{{1}, {3}, {5}}, 9};

    int total = adjust_numbers(bag.numbers);     /* bag.numbers[1] = 8, bag.numbers[2] = 9, returns 17 */
    union Number *middle = &bag.numbers[1];
    total += bump_number(middle);                /* value: 8 -> 15 */
    middle = bag.numbers + 2;
    total += middle->tag;                        /* 9 */

    return (total + bag.numbers[0].tag + bag.numbers[1].value + bag.bias) % 256;
}
