union Number {
    int value;
};

union Number make_number(int value) {
    union Number number = {value};
    return number;
}

int main(void) {
    return make_number(1);
}
