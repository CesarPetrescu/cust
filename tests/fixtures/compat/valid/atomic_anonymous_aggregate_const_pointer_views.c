typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef _Atomic(union { int value; char tag; }) AtomicAnonUnion;

typedef AtomicAnonValue *MutableAtomicAnonPtr;
typedef AtomicAnonUnion *MutableAtomicAnonUnionPtr;
typedef const AtomicAnonValue *ConstAtomicAnonView;
typedef const AtomicAnonUnion *ConstAtomicAnonUnionView;
typedef ConstAtomicAnonView const FixedConstAtomicAnonView;

AtomicAnonValue global_values[2];
AtomicAnonUnion global_choices[2];
ConstAtomicAnonView global_view = global_values;
ConstAtomicAnonUnionView global_choice_view = global_choices;

struct ConstAtomicViewHolder {
    ConstAtomicAnonView view;
    ConstAtomicAnonUnionView choice_view;
};

struct ConstAtomicViewHolder holder;

int inspect(
    ConstAtomicAnonView,
    const AtomicAnonValue[2],
    ConstAtomicAnonUnionView,
    const AtomicAnonUnion[2]
);

int inspect(
    ConstAtomicAnonView view,
    const AtomicAnonValue values[2],
    ConstAtomicAnonUnionView choice,
    const AtomicAnonUnion choices[2]
) {
    return (view == values + 1)
        + 2 * (choice == choices)
        + 4 * (sizeof(*view) == sizeof(AtomicAnonValue))
        + 8 * (sizeof(*choice) == sizeof(AtomicAnonUnion));
}

int main(void) {
    MutableAtomicAnonPtr mutable_view = global_values + 1;
    MutableAtomicAnonUnionPtr mutable_choice_view = global_choices;
    ConstAtomicAnonView const_view = mutable_view;
    ConstAtomicAnonUnionView const_choice_view = mutable_choice_view;
    FixedConstAtomicAnonView fixed_view = mutable_view;

    holder.view = mutable_view;
    holder.choice_view = mutable_choice_view;

    return inspect(const_view, global_values, const_choice_view, global_choices)
        + 16 * (global_view == global_values && global_choice_view == global_choices)
        + 32 * (sizeof(ConstAtomicAnonView) == sizeof(MutableAtomicAnonPtr)
            && sizeof(ConstAtomicAnonUnionView) == sizeof(MutableAtomicAnonUnionPtr))
        + 64 * (_Alignof(ConstAtomicAnonView) == _Alignof(MutableAtomicAnonPtr)
            && _Alignof(ConstAtomicAnonUnionView) == _Alignof(MutableAtomicAnonUnionPtr))
        + 128 * (fixed_view == mutable_view
            && holder.view == mutable_view
            && holder.choice_view == mutable_choice_view);
}
