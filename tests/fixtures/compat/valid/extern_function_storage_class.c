extern int bump(int amount);
extern void seed(int values[]);

int global = 2;

extern int total(void) {
    return 4;
}

extern int bump(int amount) {
    return amount + global + total();
}

extern void seed(int values[]) {
    values[0] = bump(3);
}

int main(void) {
    int values[2] = {0, 1};
    seed(values);
    return values[0] + values[1];
}
