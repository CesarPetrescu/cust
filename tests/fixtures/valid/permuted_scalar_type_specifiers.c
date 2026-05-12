int unsigned global = 3U;

char signed adjust(char unsigned value);

int long unsigned widen(int long signed value) {
    return (int long unsigned)value + 1UL;
}

char signed adjust(char unsigned value) {
    return (char signed)value + 2;
}

int main(void) {
    int unsigned values[2] = {4U, 5U};
    int unsigned *p = values;
    int const unsigned local = 7U;
    static int signed seen = 1;
    typedef int unsigned Count;
    Count count = 2U;
    int signed plain = -2;
    int long signed signed_long = 1L;
    int short unsigned small = 1;
    int long long unsigned huge = 1ULL;
    int unsigned total = global
        + count
        + *p
        + local
        + seen
        + sizeof(int unsigned) / sizeof(int signed)
        + sizeof(char unsigned)
        + sizeof(int long unsigned) / sizeof(int long signed)
        + sizeof(int short unsigned) / sizeof(int short signed)
        + sizeof(int long long unsigned) / sizeof(int long long signed);
    seen += 1;
    for (int unsigned i = 0U; i < 1U; i = i + 1U) {
        total += i;
    }
    return total + plain + signed_long + small + huge + adjust(3U) + widen(4L);
}
