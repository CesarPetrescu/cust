typedef const int Value, *View;
typedef _Atomic(int) AtomicInt;
typedef AtomicInt AtomicAlias;
typedef const int Alias;

int main(void) {
    Value value = 5;
    View view = &value;
    AtomicAlias atomic_value = 7;

    {
        typedef int Alias;
        _Atomic(Alias) local = 7;
        return *view + atomic_value + local;
    }
}
