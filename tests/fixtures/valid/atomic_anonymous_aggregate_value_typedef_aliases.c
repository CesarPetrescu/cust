typedef _Atomic(struct { int value; }) AtomicAnonValue, AtomicAnonCopy;
typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;

AtomicAnonCopy global_value;

struct AtomicAliasHolder {
    AtomicAnonValue value;
    AtomicAnonUnion choice;
};

struct AtomicAliasHolder holder;

int inspect(AtomicAnonValue);

int inspect(AtomicAnonValue value) {
    return sizeof(value) == sizeof(AtomicAnonValue);
}

int inspect_union(AtomicAnonUnion);

int inspect_union(AtomicAnonUnion value) {
    return 2 * (sizeof(value) == sizeof(AtomicAnonUnion));
}

int main(void) {
    AtomicAnonValue local_value = global_value;
    AtomicAnonUnion local_union = holder.choice;
    int total = inspect(local_value)
        + inspect_union(local_union)
        + (sizeof(AtomicAnonCopy) == sizeof(AtomicAnonValue))
        + (sizeof(holder.value) == sizeof(AtomicAnonValue))
        + (_Alignof(AtomicAnonUnion) == _Alignof(AtomicAnonUnion));

    {
        typedef int AtomicAnonValue;
        AtomicAnonValue shadow = 5;
        total += shadow;
    }

    return total;
}
