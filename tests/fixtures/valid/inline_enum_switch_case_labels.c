int main(void) {
    int total = 0;
    switch (sizeof(enum SwitchSize { SWITCH_SIZE = 1 })) {
        case sizeof(enum CaseSize { CASE_SIZE = 7 }):
            total = CASE_SIZE;
            break;
        default:
            total = 2;
    }
    return total;
}
