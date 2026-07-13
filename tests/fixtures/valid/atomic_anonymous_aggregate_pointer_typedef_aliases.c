typedef _Atomic(struct { int value; } *) AtomicAnonPtr, AtomicAnonCursor;
typedef _Atomic(const union { int value; char tag; } *) AtomicConstAnonPtr;
typedef AtomicAnonPtr const FixedAtomicAnonPtr;

AtomicAnonCursor global_cursor;

int is_null(AtomicAnonPtr);

int is_null(AtomicAnonPtr cursor) {
    return cursor == 0;
}

int readonly_null(AtomicConstAnonPtr cursor) {
    return cursor == 0;
}

int main(void) {
    AtomicAnonPtr local_cursor = 0;
    AtomicConstAnonPtr readonly_cursor = 0;
    FixedAtomicAnonPtr fixed_cursor = 0;
    int total = is_null(local_cursor)
        + readonly_null(readonly_cursor)
        + (global_cursor == 0)
        + (fixed_cursor == 0)
        + (sizeof(AtomicAnonPtr) == sizeof(local_cursor))
        + (sizeof(AtomicAnonCursor) == sizeof(AtomicAnonPtr))
        + (_Alignof(AtomicConstAnonPtr) == _Alignof(AtomicAnonPtr));

    {
        typedef int *AtomicAnonPtr;
        int values[1] = {5};
        AtomicAnonPtr shadow = values;
        total += *shadow;
    }

    return total;
}
