typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;

typedef AtomicAnonValue *AtomicAnonPtr, AtomicAnonArray[2];
typedef AtomicAnonUnion *AtomicAnonUnionPtr, AtomicAnonUnionArray[2];
typedef AtomicAnonPtr const ConstAtomicAnonPtr;

AtomicAnonArray global_values;
AtomicAnonUnionArray global_choices;
AtomicAnonPtr global_cursor = global_values;
AtomicAnonUnionPtr global_choice_cursor = global_choices;
_Atomic(AtomicAnonPtr) global_atomic_cursor = global_values;

struct AtomicAliasHolder {
    AtomicAnonArray values;
    AtomicAnonPtr cursor;
    AtomicAnonUnionArray choices;
    AtomicAnonUnionPtr choice_cursor;
};

struct AtomicAliasHolder holder;

int inspect(AtomicAnonPtr, AtomicAnonArray, AtomicAnonUnionPtr, AtomicAnonUnionArray);

int inspect(
    AtomicAnonPtr value,
    AtomicAnonArray values,
    AtomicAnonUnionPtr choice,
    AtomicAnonUnionArray choices
) {
    return (value == values + 1) + 2 * (choice == choices);
}

int main(void) {
    AtomicAnonArray local_values;
    AtomicAnonUnionArray local_choices;
    AtomicAnonPtr value_cursor = local_values + 1;
    AtomicAnonUnionPtr choice_cursor = local_choices;
    ConstAtomicAnonPtr fixed_cursor = value_cursor;

    holder.cursor = fixed_cursor;
    holder.choice_cursor = choice_cursor;

    return inspect(value_cursor, local_values, choice_cursor, local_choices)
        + 4 * (global_cursor == global_values && global_choice_cursor == global_choices)
        + 8 * (sizeof(AtomicAnonPtr) == sizeof(value_cursor)
            && global_atomic_cursor == global_values)
        + 16 * (sizeof(AtomicAnonArray) == 2 * sizeof(AtomicAnonValue))
        + 32 * (_Alignof(AtomicAnonArray) == _Alignof(AtomicAnonValue))
        + 64 * (sizeof(holder.values) == sizeof(AtomicAnonArray)
            && sizeof(holder.cursor) == sizeof(AtomicAnonPtr))
        + 128 * (sizeof(AtomicAnonUnionArray) == 2 * sizeof(AtomicAnonUnion)
            && _Alignof(AtomicAnonUnionArray) == _Alignof(AtomicAnonUnion));
}
