int inc(int *p) {
    *p = *p + 1;
    return *p;
}

int main() {
    int x = 6;
    int y = inc(&x);
    return x * 10 + y;
}
