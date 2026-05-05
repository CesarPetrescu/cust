int bump(int *value) {
    return ++*value;
}

int main() {
    int x = 1;
    int y = ++x;      // x=2, y=2
    int z = x++;      // x=3, z=2
    int w = --x;      // x=2, w=2
    int q = x--;      // x=1, q=2

    int values[3];
    values[0] = 4;
    values[1] = 10;
    int a = values[0]++;  // values[0]=5, a=4
    int b = ++values[0];  // values[0]=6, b=6
    int c = values[1]--;  // values[1]=9, c=10
    int d = --values[1];  // values[1]=8, d=8

    int *p = &values[2];
    *p = 20;
    int e = (*p)++;       // values[2]=21, e=20
    int f = --*p;         // values[2]=20, f=20
    int g = bump(&x);     // x=2, g=2

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
