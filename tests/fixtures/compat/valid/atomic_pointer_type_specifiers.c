struct Point { int x; };
typedef _Atomic(int *) AtomicIntPtr;

int read_second(_Atomic(int *) cursor) {
    return cursor[1];
}

int main(void) {
    int values[3] = {3, 5, 7};
    struct Point points[1] = {{11}};
    _Atomic(int *) cursor = values;
    _Atomic(const int *) readonly = values;
    _Atomic(struct Point *) point_cursor = points;
    AtomicIntPtr alias = values + 1;
    cursor = values + 2;
    return *cursor + alias[0] + read_second(values) + readonly[0] + point_cursor->x
        + (sizeof(_Atomic(int *)) == sizeof(int *))
        + (_Alignof(_Atomic(int *)) == _Alignof(int *));
}
