int main(void) {
    int flag = 0;
    int total = flag
        ? (sizeof(enum BranchA { A = 11 }) ? A : 0)
        : (sizeof(enum BranchB { B = 13 }) ? B : 0);
    total += A + B;
    total += (0 && ((enum ShortAnd { SA = 17 })0)) ? 1 : SA;
    total += (1 || (sizeof(enum ShortOr { SO = 19 }))) ? SO : 0;
    return total;
}
