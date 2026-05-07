union Number {
    int value;
    char tag;
};

struct Holder {
    union Number number;
    int bonus;
};

int read_union(union Number n) {
    n.tag = n.tag + 1;
    return n.value;
}

int main(void) {
    struct Holder holder = {{3}, 5};
    int sum = holder.number.value + holder.number.tag;
    holder.number.value = 7;
    sum = sum + holder.number.value + holder.number.tag;
    holder.number.tag = 2;
    sum = sum + holder.number.value + holder.number.tag;
    union Number copy;
    copy = holder.number;
    copy.value = copy.value + 4;
    sum = sum + copy.tag + holder.number.value;
    sum = sum + read_union(holder.number) + holder.number.value;
    union Number numbers[2] = {{4}, {6}};
    sum = sum + numbers[0].value + numbers[0].tag;
    numbers[1].value = 9;
    sum = sum + numbers[1].tag;
    sum = sum + holder.bonus;
    return sum;
}
