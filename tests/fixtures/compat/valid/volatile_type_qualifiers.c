volatile int total = 3;
const volatile char marker = 4;

struct Point {
    int x;
    int y;
};

volatile struct Point origin = {5, 6};

typedef volatile int VInt;
typedef volatile struct Point VPoint;

int add(volatile int left, const volatile int *right) {
    volatile int sum = left + *right;
    return sum;
}

int main(void) {
    volatile int local = 7;
    volatile int values[2] = {8, 9};
    volatile int *cursor = values;
    volatile int * volatile slot = values;
    VInt alias_value = 10;
    VPoint alias_point = {11, 12};
    volatile struct Point copy = origin;
    return total + marker + local + cursor[1] + slot[0] + alias_value + alias_point.y + copy.x + add(2, &values[0])
        + sizeof(volatile char) + sizeof(const volatile char[2]) + (volatile int)3;
}
