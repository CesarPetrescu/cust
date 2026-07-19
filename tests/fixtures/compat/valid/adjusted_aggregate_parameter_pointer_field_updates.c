struct MutableInner {
    int *cursor;
};

struct NamedHolder {
    struct MutableInner nested;
};

struct AnonymousConstViewHolder {
    struct {
        const int *cursor;
    } nested;
};

union Choice {
    struct MutableInner nested;
    int marker;
};

int replace_named(
    struct NamedHolder holders[const 1],
    int *replacement
) {
    int outer = 0;
    int rhs = 0;
    int *result = holders[outer++].nested.cursor = replacement + (rhs++, 1);
    return result[1] + holders[0].nested.cursor[0] + outer + rhs;
}

int shift_union(union Choice choices[]) {
    int outer = 0;
    int rhs = 0;
    int *after_compound =
        (choices[outer++].nested.cursor += (rhs++, 1));
    int *old = choices[0].nested.cursor++;
    int *updated = --choices[0].nested.cursor;
    return after_compound[0] + old[0] + updated[1] + outer + rhs;
}

int replace_anonymous_const_view(
    struct AnonymousConstViewHolder holders[],
    const int *replacement
) {
    const int *result = holders[0].nested.cursor = replacement;
    return result[0] + holders[0].nested.cursor[1];
}

int inspect_sizeof(union Choice choices[], int *replacement) {
    int ok = 0;
    ok += sizeof(choices[0].nested.cursor) == sizeof(int *);
    ok += sizeof(choices[0].nested.cursor + 1) == sizeof(int *);
    ok += sizeof(replacement) == sizeof(int *);
    return ok + (choices[0].nested.cursor[0] == 2)
        + (replacement[0] == 5) + 1;
}

int main(void) {
    int original[2] = {2, 3};
    int replacement[3] = {5, 7, 11};
    int shift_values[3] = {2, 3, 5};
    int measured_values[2] = {2, 3};
    const int fixed[2] = {11, 13};
    struct NamedHolder named[1] = {{.nested = {original}}};
    struct AnonymousConstViewHolder anonymous[1] = {
        {.nested = {original}}
    };
    union Choice shifted[1] = {{.nested = {shift_values}}};
    union Choice measured[1] = {{.nested = {measured_values}}};

    return replace_named(named, replacement)
        + shift_union(shifted)
        + replace_anonymous_const_view(anonymous, fixed)
        + inspect_sizeof(measured, replacement);
}
