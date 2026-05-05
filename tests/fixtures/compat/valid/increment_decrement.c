int bump(int *value) {
    return ++*value;
}

int main() {
    int x = 1;
    int y = ++x;
    int z = x++;
    int w = --x;
    int q = x--;

    int values[3];
    values[0] = 4;
    values[1] = 10;
    int a = values[0]++;
    int b = ++values[0];
    int c = values[1]--;
    int d = --values[1];

    int *p = &values[2];
    *p = 20;
    int e = (*p)++;
    int f = --*p;
    int g = bump(&x);

    int loop_sum = 0;
    for (int i = 0; i < 4; i++) {
        loop_sum += i;
    }

    int down = 3;
    while (down-- > 0) {
        loop_sum += down;
    }

    return x + y + z + w + q + a + b + c + d + e + f + g + loop_sum;
}
