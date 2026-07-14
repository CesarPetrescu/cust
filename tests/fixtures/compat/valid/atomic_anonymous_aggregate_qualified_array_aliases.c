typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;

typedef const AtomicAnonValue ConstAtomicAnonValue;
typedef volatile AtomicAnonValue VolatileAtomicAnonValue;
typedef const AtomicAnonUnion ConstAtomicAnonUnion;
typedef volatile AtomicAnonUnion VolatileAtomicAnonUnion;

typedef ConstAtomicAnonValue ConstAtomicAnonArray[2];
typedef VolatileAtomicAnonValue VolatileAtomicAnonArray[2];
typedef ConstAtomicAnonUnion ConstAtomicAnonUnionArray[2];
typedef VolatileAtomicAnonUnion VolatileAtomicAnonUnionArray[2];

ConstAtomicAnonArray global_const_values;
VolatileAtomicAnonArray global_volatile_values;
ConstAtomicAnonUnionArray global_const_choices;
VolatileAtomicAnonUnionArray global_volatile_choices;

struct QualifiedAtomicArrayAliasHolder {
    ConstAtomicAnonArray const_values;
    VolatileAtomicAnonArray volatile_values;
    ConstAtomicAnonUnionArray const_choices;
    VolatileAtomicAnonUnionArray volatile_choices;
};

struct QualifiedAtomicArrayAliasHolder holder;

int inspect(
    ConstAtomicAnonArray,
    ConstAtomicAnonValue *,
    VolatileAtomicAnonArray,
    VolatileAtomicAnonValue *,
    ConstAtomicAnonUnionArray,
    ConstAtomicAnonUnion *,
    VolatileAtomicAnonUnionArray,
    VolatileAtomicAnonUnion *
);

int inspect(
    ConstAtomicAnonArray const_values,
    ConstAtomicAnonValue *const_expected,
    VolatileAtomicAnonArray volatile_values,
    VolatileAtomicAnonValue *volatile_expected,
    ConstAtomicAnonUnionArray const_choices,
    ConstAtomicAnonUnion *const_choice_expected,
    VolatileAtomicAnonUnionArray volatile_choices,
    VolatileAtomicAnonUnion *volatile_choice_expected
) {
    const_values = const_expected;
    volatile_values = volatile_expected;
    const_choices = const_choice_expected;
    volatile_choices = volatile_choice_expected;
    return const_values == const_expected
        && volatile_values == volatile_expected
        && const_choices == const_choice_expected
        && volatile_choices == volatile_choice_expected;
}

int main(void) {
    int shadow_ok = 0;
    static ConstAtomicAnonArray local_const_values;
    VolatileAtomicAnonArray local_volatile_values;
    static ConstAtomicAnonUnionArray local_const_choices;
    VolatileAtomicAnonUnionArray local_volatile_choices;
    ConstAtomicAnonValue *const_view = local_const_values + 1;
    VolatileAtomicAnonValue *volatile_view = local_volatile_values + 1;
    ConstAtomicAnonUnion *const_choice_view = local_const_choices;
    VolatileAtomicAnonUnion *volatile_choice_view = local_volatile_choices;

    {
        typedef int ConstAtomicAnonArray;
        typedef char ConstAtomicAnonUnionArray;
        ConstAtomicAnonArray value = 5;
        ConstAtomicAnonUnionArray tag = 2;
        shadow_ok = value + tag == 7;
    }

    return inspect(
        global_const_values,
        global_const_values,
        global_volatile_values,
        global_volatile_values,
        global_const_choices,
        global_const_choices,
        global_volatile_choices,
        global_volatile_choices
    )
        + 2 * inspect(
            local_const_values,
            local_const_values,
            local_volatile_values,
            local_volatile_values,
            local_const_choices,
            local_const_choices,
            local_volatile_choices,
            local_volatile_choices
        )
        + 4 * (sizeof(ConstAtomicAnonArray) == 2 * sizeof(AtomicAnonValue)
            && sizeof(VolatileAtomicAnonArray) == 2 * sizeof(AtomicAnonValue)
            && sizeof(ConstAtomicAnonUnionArray) == 2 * sizeof(AtomicAnonUnion)
            && sizeof(VolatileAtomicAnonUnionArray) == 2 * sizeof(AtomicAnonUnion))
        + 8 * (sizeof(holder.const_values) == sizeof(ConstAtomicAnonArray)
            && sizeof(holder.volatile_values) == sizeof(VolatileAtomicAnonArray)
            && sizeof(holder.const_choices) == sizeof(ConstAtomicAnonUnionArray)
            && sizeof(holder.volatile_choices) == sizeof(VolatileAtomicAnonUnionArray))
        + 16 * (_Alignof(ConstAtomicAnonArray) == _Alignof(AtomicAnonValue)
            && _Alignof(VolatileAtomicAnonArray) == _Alignof(AtomicAnonValue)
            && _Alignof(ConstAtomicAnonUnionArray) == _Alignof(AtomicAnonUnion)
            && _Alignof(VolatileAtomicAnonUnionArray) == _Alignof(AtomicAnonUnion))
        + 32 * (const_view == local_const_values + 1
            && volatile_view == local_volatile_values + 1
            && const_choice_view == local_const_choices
            && volatile_choice_view == local_volatile_choices)
        + 64 * shadow_ok
        + 128 * (sizeof(local_const_values) == sizeof(ConstAtomicAnonArray)
            && sizeof(local_volatile_values) == sizeof(VolatileAtomicAnonArray)
            && sizeof(local_const_choices) == sizeof(ConstAtomicAnonUnionArray)
            && sizeof(local_volatile_choices) == sizeof(VolatileAtomicAnonUnionArray));
}
