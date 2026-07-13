_Atomic(enum GlobalAtomic {
    GLOBAL_ATOMIC = 4
}) global_atomic = GLOBAL_ATOMIC;

struct AtomicHolder {
    _Atomic(enum FieldAtomic {
        FIELD_ATOMIC = 5
    }) value;
};

int read_atomic(_Atomic(enum ParamAtomic {
    PARAM_ATOMIC = 6
}) value) {
    return value + PARAM_ATOMIC;
}

int main(void) {
    struct AtomicHolder holder = {FIELD_ATOMIC};
    _Atomic(enum LocalAtomic {
        LOCAL_ATOMIC = 7
    }) local = LOCAL_ATOMIC;
    int size_ok = sizeof(_Atomic(enum SizeAtomic {
        SIZE_ATOMIC = 8
    })) == sizeof(_Atomic(enum GlobalAtomic));
    int align_ok = _Alignof(_Atomic(enum AlignAtomic {
        ALIGN_ATOMIC = 9
    })) == _Alignof(_Atomic(enum GlobalAtomic));

    return global_atomic + holder.value + read_atomic(local) + local
        + size_ok + align_ok + SIZE_ATOMIC + ALIGN_ATOMIC;
}
