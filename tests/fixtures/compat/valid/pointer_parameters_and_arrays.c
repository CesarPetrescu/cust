int inc(int *p) {
    *p = *p + 1;
    return *p;
}

int sum3(int *values) {
    values[1] = values[1] + 5;
    return values[0] + values[1] + values[2];
}

int main() {
    int x = 6;
    int y = inc(&x);
    int values[3];
    values[0] = 1;
    values[1] = 2;
    values[2] = 3;
    return x * 10 + y + sum3(values);
}
