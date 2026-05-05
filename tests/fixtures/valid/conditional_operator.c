int choose(int cond, int left, int right) {
    return cond ? left : right;
}

int main() {
    int zero = 0;
    int x = 0;
    int y = 0;
    int values[3];
    values[0] = 2;
    values[1] = 4;
    values[2] = 8;
    int *p = &values[1];

    int result = 1 ? 10 : 1 / zero;
    result = result + (0 ? 1 / zero : 20);
    result = result + (zero ? 100 : 0 ? 200 : 30);
    result = result + (1 ? (x = 5) : (x = 99));
    result = result + x;
    result = result + (0 ? (y = 99) : (y = 7));
    result = result + y;
    result = result + (p ? *p : 0);
    p = 0;
    result = result + (p ? 1000 : values[2]);

    return result;
}
