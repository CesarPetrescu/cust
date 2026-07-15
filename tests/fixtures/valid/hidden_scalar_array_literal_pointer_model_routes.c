typedef const int ConstInts[4];
typedef const char ConstChars[4];

int main(void) {
    int *mutable_int = (int[4]){11, 12, 13, 14};
    char *mutable_char = (char[4]){51, 52, 53, 54};
    const int *const_int = (ConstInts){31, 32, 33, 34};
    const char *const_char = (ConstChars){71, 72, 73, 74};
    char *text = "abc";

    int *i0 = mutable_int + 1;
    int *i1 = 0 ? mutable_int + 3 : mutable_int + 2;
    int marker = 0;
    int *i2 = (marker += (mutable_int + 3) - (mutable_int + 1), mutable_int + 3);
    const int *ci = const_int + 2;
    char *c0 = mutable_char + 1;
    const char *cc = const_char + 2;
    char *s0 = text + 1;

    int differences = (mutable_int + 3) - mutable_int;
    differences += (const_int + 2) - const_int;
    differences += (text + 2) - text;
    int comparisons = i0 == mutable_int + 1;
    comparisons += i2 > i0;
    comparisons += text + 2 > text;

    mutable_int[0] = 5;
    mutable_char[0] = 6;

    return (*i0 + *i1 + *i2 + *ci + *c0 + *cc + *s0 + differences +
            comparisons + mutable_int[0] + mutable_char[0] + marker - 2) % 251;
}
