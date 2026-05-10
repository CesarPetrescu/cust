unsigned int global = 3U;

signed int add(unsigned int lhs, signed int rhs);

int bump(unsigned int *slot) {
    *slot += 2;
    return *slot;
}

signed int add(unsigned int lhs, signed int rhs) {
    return (signed int)lhs + rhs;
}

int main(void) {
    unsigned int values[2] = {4U, 5U};
    unsigned int *p = values;
    const unsigned int local = 7U;
    static signed int seen = 1;
    typedef unsigned int Count;
    Count count = 2U;
    unsigned bare = 1U;
    signed plain = -2;
    signed int total = add(global, count)
        + bump(p)
        + sizeof(unsigned int) / sizeof(signed int)
        + sizeof(const unsigned int[2]) / sizeof(unsigned int)
        + local
        + seen;
    seen += 1;
    for (unsigned int i = 0U; i < 1U; i = i + 1U) {
        total += i;
    }
    return total + bare + plain + sizeof(unsigned) / sizeof(signed);
}
