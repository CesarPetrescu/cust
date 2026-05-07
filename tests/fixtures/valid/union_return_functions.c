union Number {
    int value;
    char tag;
};

union Number make_number(int value);
int read_number(union Number n);

union Number make_number(int value) {
    union Number n = {value};
    return n;
}

int read_number(union Number n) {
    n.value += 2;
    return n.tag;
}

int main(void) {
    union Number n;
    n = make_number(5);
    int sum = n.value + read_number(n);
    sum = sum + n.value;

    union Number n2;
    n2 = make_number(9);
    sum = sum + n2.tag;

    return sum;
}
