union Number {
    int value;
    char tag;
};

int main(void) {
    union Number n = {5};
    int sum = n.value + n.tag;
    n.value = 40;
    sum = sum + n.value + n.tag;
    n.tag = 2;
    sum = sum + n.value + n.tag;
    sum = sum + sizeof(n) + sizeof(n.tag);
    return sum;
}
