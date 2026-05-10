long global_long = 5L;
short global_short = 2;

long add_long(unsigned long lhs, signed long rhs);
short bump_short(short *slot) {
    *slot += 3;
    return *slot;
}

long add_long(unsigned long lhs, signed long rhs) {
    return (long)lhs + (short)rhs;
}

int main(void) {
    long values[2] = {4L, 6L};
    long *p = values;
    const short local_short = 7;
    static unsigned short seen = 1;
    typedef signed short Small;
    typedef unsigned long Big;
    Small count = 2;
    Big wide = 3UL;
    signed short signed_local = -1;
    unsigned short bare_short = 1;
    long int explicit_long = 1L;
    short int explicit_short = 1;
    signed long int signed_long_int = 1L;
    unsigned short int unsigned_short_int = 1;
    long total = add_long(global_long, count)
        + bump_short(&count)
        + p[1]
        + local_short
        + seen
        + wide
        + explicit_long
        + explicit_short
        + signed_long_int
        + unsigned_short_int;
    seen += 1;
    for (short i = 0; i < 1; i = i + 1) {
        total += i;
    }
    return total + global_short + signed_local + bare_short;
}
