_Atomic(struct {
    int value;
} *) global_cursor;

int is_null(_Atomic(struct {
    int value;
} *) cursor) {
    return cursor == 0;
}

int main(void) {
    _Atomic(struct {
        int value;
    } *) local_cursor = 0;
    _Atomic(const union {
        int value;
        char tag;
    } *) readonly_cursor = 0;

    return (global_cursor == 0)
        + (local_cursor == 0)
        + (readonly_cursor == 0)
        + is_null(0)
        + (sizeof(local_cursor) == sizeof(int *))
        + (sizeof(_Atomic(struct { int value; } *)) == sizeof(int *))
        + (_Alignof(_Atomic(const union { int value; char tag; } *)) == _Alignof(int *));
}
