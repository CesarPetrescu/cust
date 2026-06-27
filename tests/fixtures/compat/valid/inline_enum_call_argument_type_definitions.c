int take(int left, int middle, int right) {
    return left + middle + right;
}

int main(void) {
    int total = take(
        sizeof(enum ArgSize { ARG_SIZE = 4 }) == sizeof(enum ArgSize) ? ARG_SIZE : 0,
        _Alignof(enum ArgAlign { ARG_ALIGN = 6 }) == _Alignof(enum ArgAlign) ? ARG_ALIGN : 0,
        (enum ArgCast { ARG_CAST = 8 })0 + ARG_CAST);
    int after = ARG_SIZE + ARG_ALIGN + ARG_CAST;
    return total + after;
}
