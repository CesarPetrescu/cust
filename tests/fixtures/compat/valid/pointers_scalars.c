int main() {
    int x = 3;
    int y = 4;
    int *p = &x;
    *p = *p + 2;
    p = &y;
    return x * 10 + *p;
}
