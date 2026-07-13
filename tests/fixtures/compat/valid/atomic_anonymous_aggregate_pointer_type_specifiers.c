_Atomic(struct {
    int value;
} *) global_cursor;

struct AtomicAnonymousPointerHolder {
    _Atomic(const union {
        int value;
        char tag;
    } *) view;
};

struct AtomicAnonymousPointerHolder holder;

int main(void) {
    _Atomic(struct {
        int value;
    } *) local_cursor = 0;
    _Atomic(const union {
        int value;
        char tag;
    } *) readonly_cursor = 0;

    return (sizeof(global_cursor) == sizeof(int *))
        + (sizeof(holder.view) == sizeof(int *))
        + (sizeof(local_cursor) == sizeof(int *))
        + (sizeof(readonly_cursor) == sizeof(int *))
        + (sizeof(_Atomic(struct { int value; } *)) == sizeof(int *))
        + (_Alignof(_Atomic(const union { int value; char tag; } *)) == _Alignof(int *));
}
