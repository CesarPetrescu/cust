typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;

typedef const AtomicAnonValue ConstAtomicAnonValue;
typedef volatile AtomicAnonValue VolatileAtomicAnonValue;
typedef const AtomicAnonUnion ConstAtomicAnonUnion;
typedef volatile AtomicAnonUnion VolatileAtomicAnonUnion;
typedef ConstAtomicAnonValue ChainedConstAtomicAnonValue;
typedef ConstAtomicAnonUnion ChainedConstAtomicAnonUnion;

AtomicAnonValue global_values[2];
AtomicAnonUnion global_choices[2];
ConstAtomicAnonValue global_const_values[2];
ConstAtomicAnonUnion global_const_choices[2];

struct QualifiedAtomicAliasHolder {
    ConstAtomicAnonValue value;
    VolatileAtomicAnonValue volatile_value;
    ConstAtomicAnonUnion choice;
    VolatileAtomicAnonUnion volatile_choice;
    ChainedConstAtomicAnonValue values[2];
    ChainedConstAtomicAnonUnion choices[2];
    ConstAtomicAnonValue *view;
    ConstAtomicAnonUnion *choice_view;
};

struct QualifiedAtomicAliasHolder holder;

int inspect(
    ConstAtomicAnonValue,
    ChainedConstAtomicAnonValue,
    VolatileAtomicAnonValue,
    ConstAtomicAnonUnion,
    ChainedConstAtomicAnonUnion,
    VolatileAtomicAnonUnion
);

int inspect(
    ConstAtomicAnonValue value,
    ChainedConstAtomicAnonValue chained,
    VolatileAtomicAnonValue volatile_value,
    ConstAtomicAnonUnion choice,
    ChainedConstAtomicAnonUnion chained_choice,
    VolatileAtomicAnonUnion volatile_choice
) {
    return (sizeof(value) == sizeof(AtomicAnonValue))
        + 2 * (sizeof(chained) == sizeof(AtomicAnonValue))
        + 4 * (sizeof(volatile_value) == sizeof(AtomicAnonValue))
        + 8 * (sizeof(choice) == sizeof(AtomicAnonUnion)
            && sizeof(chained_choice) == sizeof(AtomicAnonUnion)
            && sizeof(volatile_choice) == sizeof(AtomicAnonUnion));
}

int main(void) {
    int shadow_ok = 0;
    ConstAtomicAnonValue local_value = global_values[0];
    ChainedConstAtomicAnonValue chained_value = local_value;
    VolatileAtomicAnonValue volatile_value = global_values[1];
    ConstAtomicAnonUnion local_choice = global_choices[0];
    ChainedConstAtomicAnonUnion chained_choice = local_choice;
    VolatileAtomicAnonUnion volatile_choice = global_choices[1];
    ConstAtomicAnonValue *view = global_values + 1;
    ConstAtomicAnonUnion *choice_view = global_choices;

    holder.view = global_values;
    holder.choice_view = global_choices;

    {
        typedef int ConstAtomicAnonValue;
        typedef char VolatileAtomicAnonValue;
        ConstAtomicAnonValue shadow = 5;
        VolatileAtomicAnonValue marker = 2;
        shadow_ok = shadow + marker == 7;
    }

    return inspect(
        local_value,
        chained_value,
        volatile_value,
        local_choice,
        chained_choice,
        volatile_choice
    )
        + 16 * (sizeof(global_const_values) == 2 * sizeof(AtomicAnonValue)
            && sizeof(global_const_choices) == 2 * sizeof(AtomicAnonUnion)
            && sizeof(holder.value) == sizeof(ConstAtomicAnonValue)
            && sizeof(holder.choice) == sizeof(ConstAtomicAnonUnion))
        + 32 * (_Alignof(ConstAtomicAnonValue) == _Alignof(AtomicAnonValue)
            && _Alignof(VolatileAtomicAnonValue) == _Alignof(AtomicAnonValue)
            && _Alignof(ConstAtomicAnonUnion) == _Alignof(AtomicAnonUnion)
            && _Alignof(VolatileAtomicAnonUnion) == _Alignof(AtomicAnonUnion))
        + 64 * (view == global_values + 1
            && choice_view == global_choices
            && holder.view == global_values
            && holder.choice_view == global_choices)
        + 128 * (sizeof(holder.values) == 2 * sizeof(AtomicAnonValue)
            && sizeof(holder.choices) == 2 * sizeof(AtomicAnonUnion)
            && sizeof(chained_value) == sizeof(ConstAtomicAnonValue)
            && sizeof(chained_choice) == sizeof(ConstAtomicAnonUnion)
            && shadow_ok);
}
