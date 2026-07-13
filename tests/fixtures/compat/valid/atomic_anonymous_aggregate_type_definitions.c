_Atomic(struct {
    int value;
}) global_atomic;

struct AtomicAnonymousHolder {
    _Atomic(struct {
        int value;
    }) field;
    _Atomic(union {
        int value;
        char tag;
    }) choice;
};

struct AtomicAnonymousHolder holder;

int main(void) {
    _Atomic(struct {
        int value;
    }) local_atomic;
    _Atomic(union {
        int value;
        char tag;
    }) local_choice;
    int size_ok = sizeof(_Atomic(struct {
        int value;
    })) == sizeof(_Atomic(struct {
        int value;
    }));
    int align_ok = _Alignof(_Atomic(union {
        int value;
        char tag;
    })) == _Alignof(_Atomic(union {
        int value;
        char tag;
    }));

    return (sizeof(global_atomic) == sizeof(_Atomic(struct { int value; })))
        + (sizeof(holder.field) == sizeof(_Atomic(struct { int value; })))
        + (sizeof(holder.choice) == sizeof(_Atomic(union { int value; char tag; })))
        + (sizeof(local_atomic) == sizeof(_Atomic(struct { int value; })))
        + (sizeof(local_choice) == sizeof(_Atomic(union { int value; char tag; })))
        + size_ok + align_ok;
}
