union Number {
    int value;
    char tag;
};

int main(void) {
    union Number n = {1, 2};
    return n.value;
}
