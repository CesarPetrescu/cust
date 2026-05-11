_Atomic int total = 3;
_Atomic(int) wrapped_global = 4;

typedef _Atomic int AtomicInt;
typedef _Atomic(char) AtomicChar;

int add(_Atomic int left, _Atomic(int) *right) {
    _Atomic int sum = left + *right;
    return sum;
}

_Atomic(int) passthrough(_Atomic(int) input) {
    return input + 1;
}

int main(void) {
    _Atomic int local = 5;
    _Atomic(int) values[2] = {6, 7};
    _Atomic(int) *cursor = values;
    AtomicInt alias_value = 8;
    AtomicChar marker = 9;
    for (_Atomic int i = 0; i < 2; i = i + 1)
        local = local + i;
    return total + wrapped_global + local + cursor[1] + alias_value + marker + add(2, &values[0])
        + passthrough(5) + (sizeof(_Atomic char) == sizeof(char));
}
