int main(void) {
    int decimal = 40u + 2U;
    int long_value = 10l + 11L;
    int long_long_value = 12ll + 13LL;
    int mixed = 0x10UL + 07lu + 5uL + 6LLU;
    return decimal + long_value + long_long_value + mixed;
}
