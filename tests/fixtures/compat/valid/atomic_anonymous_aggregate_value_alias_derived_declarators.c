typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;

AtomicAnonValue global_values[2];
AtomicAnonUnion global_choices[2];

struct AtomicDerivedHolder {
    AtomicAnonValue values[2];
    AtomicAnonValue *cursor;
    AtomicAnonUnion choices[2];
    AtomicAnonUnion *choice_cursor;
};

struct AtomicDerivedHolder holder;

int inspect(AtomicAnonValue *, AtomicAnonUnion[2]);

int inspect(AtomicAnonValue *value, AtomicAnonUnion choices[2]) {
    return (sizeof(*value) == sizeof(AtomicAnonValue))
        + 2 * (sizeof(choices[0]) == sizeof(AtomicAnonUnion));
}

int main(void) {
    AtomicAnonValue *value_cursor = global_values + 1;
    AtomicAnonUnion *choice_cursor = global_choices;
    holder.cursor = value_cursor;
    holder.choice_cursor = choice_cursor;

    return inspect(value_cursor, choice_cursor)
        + 4 * (sizeof(global_values) == 2 * sizeof(AtomicAnonValue))
        + 8 * (sizeof(holder.values) == 2 * sizeof(AtomicAnonValue))
        + 16 * (sizeof(holder.cursor) == sizeof(AtomicAnonValue *))
        + 32 * (_Alignof(AtomicAnonValue[2]) == _Alignof(AtomicAnonValue))
        + 64 * (_Alignof(AtomicAnonUnion[2]) == _Alignof(AtomicAnonUnion));
}
