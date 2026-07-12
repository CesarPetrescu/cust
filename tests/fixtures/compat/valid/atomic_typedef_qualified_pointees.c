typedef const int ConstInt;
typedef const int *ConstView;
typedef volatile int *VolatileView;
typedef const int Shadowed;

int main(void) {
    int value = 7;
    _Atomic(ConstView) left = &value;
    _Atomic(VolatileView) right = &value;
    _Atomic(ConstInt *) extra = &value;
    {
        typedef int Shadowed;
        _Atomic(Shadowed) plain = 1;
        return *left + *right + *extra + plain;
    }
}
