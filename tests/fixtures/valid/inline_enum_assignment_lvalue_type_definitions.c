struct Box {
    int value;
    int values[2];
};

int main(void) {
    int total = 0;
    int values[2] = {0, 0};
    struct Box box = {0, {0, 0}};
    int target = 0;
    int *slot = &target;

    values[0] = _Alignof(enum ArrayAlign { ARRAY_ALIGN = 3 }) + ARRAY_ALIGN;
    total = total + ARRAY_ALIGN + (values[0] == _Alignof(enum ArrayAlign) + ARRAY_ALIGN);

    box.value = _Alignof(enum FieldAlign { FIELD_ALIGN = 5 }) + FIELD_ALIGN;
    total = total + FIELD_ALIGN + (box.value == _Alignof(enum FieldAlign) + FIELD_ALIGN);

    box.values[1] = _Alignof(enum FieldArrayAlign { FIELD_ARRAY_ALIGN = 7 })
        + FIELD_ARRAY_ALIGN;
    total = total + FIELD_ARRAY_ALIGN
        + (box.values[1] == _Alignof(enum FieldArrayAlign) + FIELD_ARRAY_ALIGN);

    *slot = _Alignof(enum DerefAlign { DEREF_ALIGN = 11 }) + DEREF_ALIGN;
    total = total + DEREF_ALIGN + (target == _Alignof(enum DerefAlign) + DEREF_ALIGN);

    values[1] += _Alignof(enum ArrayCompoundAlign { ARRAY_COMPOUND_ALIGN = 13 })
        + ARRAY_COMPOUND_ALIGN;
    total = total + ARRAY_COMPOUND_ALIGN
        + (values[1] == _Alignof(enum ArrayCompoundAlign) + ARRAY_COMPOUND_ALIGN);

    box.value += _Alignof(enum FieldCompoundAlign { FIELD_COMPOUND_ALIGN = 17 })
        + FIELD_COMPOUND_ALIGN;
    total = total + FIELD_COMPOUND_ALIGN
        + (box.value == _Alignof(enum FieldAlign) + FIELD_ALIGN
            + _Alignof(enum FieldCompoundAlign) + FIELD_COMPOUND_ALIGN);

    *slot += _Alignof(enum DerefCompoundAlign { DEREF_COMPOUND_ALIGN = 19 })
        + DEREF_COMPOUND_ALIGN;
    total = total + DEREF_COMPOUND_ALIGN
        + (target == _Alignof(enum DerefAlign) + DEREF_ALIGN
            + _Alignof(enum DerefCompoundAlign) + DEREF_COMPOUND_ALIGN);

    return total;
}
