struct Point {
    int x;
};

typedef const struct Point ConstPoint;
typedef ConstPoint *View;
typedef _Atomic(struct Point) AtomicPoint;
typedef const struct Point Alias;

int read(_Atomic(View) view, AtomicPoint *slot) {
    return (*view).x + (sizeof(*slot) == sizeof(AtomicPoint));
}

int main(void) {
    struct Point point = {7};
    View view = &point;
    AtomicPoint slot;

    {
        typedef struct Point Alias;
        _Atomic(Alias) local;
        return read(view, &slot) + (sizeof(local) == sizeof(Alias));
    }
}