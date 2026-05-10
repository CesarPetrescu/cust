unsigned char global = 3U;

signed char add(unsigned char lhs, signed char rhs);

char bump(unsigned char *slot) {
    *slot += 2;
    return *slot;
}

signed char add(unsigned char lhs, signed char rhs) {
    return (signed char)lhs + rhs;
}

int main(void) {
    unsigned char values[2] = {4U, 5U};
    unsigned char *p = values;
    const unsigned char local = 7U;
    static signed char seen = 1;
    typedef unsigned char Byte;
    Byte count = 2U;
    unsigned char bare = 1U;
    signed char signed_local = -2;
    signed char total = add(global, count)
        + bump(p)
        + sizeof(unsigned char)
        + sizeof(const signed char[3]) / sizeof(signed char)
        + local
        + seen;
    seen += 1;
    for (signed char i = 0; i < 1; i = i + 1) {
        total += i;
    }
    return total + bare + signed_local + sizeof(unsigned char *) / sizeof(signed char *);
}
