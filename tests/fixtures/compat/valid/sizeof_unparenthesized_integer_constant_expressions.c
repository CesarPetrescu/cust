enum Sizes {
    WORD_BYTES = sizeof "abc",
    UNARY_INT_BYTES = sizeof +1,
    NESTED_SIZEOF_BYTES = sizeof sizeof(int)
};

int main(void) {
    int total = 0;
    switch (sizeof "abc") {
    case sizeof "":
        total = total + 100;
        break;
    case sizeof "abc":
        total = total + WORD_BYTES;
        break;
    default:
        total = total + 1000;
    }

    return total + UNARY_INT_BYTES + NESTED_SIZEOF_BYTES;
}
