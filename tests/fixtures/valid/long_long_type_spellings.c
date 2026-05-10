long long global_long_long = 5LL;
unsigned long long global_unsigned_long_long = 3ULL;

long long add_long_long(signed long long lhs, unsigned long long rhs);
long long bump_long_long(long long *slot) {
    *slot += 2LL;
    return *slot;
}

long long add_long_long(signed long long lhs, unsigned long long rhs) {
    return (long long)lhs + (unsigned long long)rhs;
}

int main(void) {
    long long values[2] = {4LL, 6LL};
    long long *p = values;
    const unsigned long long local_const = 7ULL;
    static signed long long seen = 1LL;
    typedef signed long long SignedWide;
    typedef unsigned long long UnsignedWide;
    SignedWide count = 2LL;
    UnsignedWide wide = 3ULL;
    signed long long explicit_signed = 1LL;
    unsigned long long explicit_unsigned = 1ULL;
    long long int explicit_int_suffix = 1LL;
    signed long long int signed_int_suffix = 1LL;
    unsigned long long int unsigned_int_suffix = 1ULL;
    long long total = add_long_long(global_long_long, count)
        + bump_long_long(&count)
        + p[1]
        + sizeof(long long)
        + sizeof(const unsigned long long[3]) / sizeof(long long)
        + local_const
        + seen
        + wide
        + explicit_signed
        + explicit_unsigned
        + explicit_int_suffix
        + signed_int_suffix
        + unsigned_int_suffix;
    seen += 1LL;
    for (long long i = 0LL; i < 1LL; i = i + 1LL) {
        total += i;
    }
    return total + global_unsigned_long_long + (int)global_long_long + sizeof(long long *) / sizeof(unsigned long long *);
}
