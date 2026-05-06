int helper(void) {
    return 4;
}

void bump(int *value) {
    *value += helper();
}

int use_prototype(void);

int use_prototype(void) {
    int total = 2;
    bump(&total);
    return total;
}

int main(void) {
    return use_prototype();
}
