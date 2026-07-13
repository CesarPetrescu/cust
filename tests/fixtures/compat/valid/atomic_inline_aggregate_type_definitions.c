_Atomic(struct GlobalAtomic {
    int value;
}) global_atomic;

struct AtomicHolder {
    _Atomic(struct FieldAtomic {
        int value;
    }) field;
    _Atomic(union FieldChoice {
        int value;
        char tag;
    }) choice;
};

struct AtomicHolder holder;

int main(void) {
    _Atomic(struct LocalAtomic {
        int value;
    }) local_atomic;
    _Atomic(union LocalChoice {
        int value;
        char tag;
    }) local_choice;
    int size_ok = sizeof(_Atomic(struct SizeAtomic {
        int value;
    })) == sizeof(struct SizeAtomic);
    int align_ok = _Alignof(_Atomic(union AlignAtomic {
        int value;
        char tag;
    })) == _Alignof(union AlignAtomic);

    return (sizeof(global_atomic) == sizeof(struct GlobalAtomic))
        + (sizeof(holder.field) == sizeof(struct FieldAtomic))
        + (sizeof(holder.choice) == sizeof(union FieldChoice))
        + (sizeof(local_atomic) == sizeof(struct LocalAtomic))
        + (sizeof(local_choice) == sizeof(union LocalChoice))
        + size_ok + align_ok;
}
