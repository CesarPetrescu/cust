int bump(int *slot) {
    *slot = *slot + 1;
    return *slot;
}

int choose(int flag) {
    return flag ? (int){11} : (int){22};
}

int add(int left, int right) {
    return left + right;
}

int main(void) {
    int calls = 0;
    int total = 0;

    total += (int){bump(&calls)};
    total += (int){bump(&calls),};
    total += (char){65} == 'A';
    total += (int){3} + (int){4};
    total += choose(0);
    total += add((int){bump(&calls)}, (int){5});
    total += sizeof((char){bump(&calls)}) == 1;

    return total + calls;
}
