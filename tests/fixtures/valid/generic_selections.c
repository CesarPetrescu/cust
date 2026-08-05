struct Pair {
    int x;
};

int bump(int *value) {
    *value += 1;
    return *value;
}

int identity(int value) {
    return value;
}

int scalar_only(void);
int *pointer_only(void);
struct Pair pair_only(void);

int main(void) {
    int calls = 0;
    int value = 4;
    char letter = 'a';
    _Bool flag = 1;
    int *pointer = &value;
    const int *const_pointer = &value;
    struct Pair pair = {6};
    int *selected_pointer = _Generic(value, int: pointer, default: pointer);
    struct Pair selected_pair = _Generic(value, int: pair, default: (struct Pair){0});
    int selected_values[2] = {
        _Generic(value, int: 1, default: 100),
        _Generic(letter, char: 2, default: 100)
    };
    int total = 0;

    total += _Generic(value, char: 100, int: 1, default: 200);
    total += _Generic(letter, char: 2, default: 200);
    total += _Generic(flag, _Bool: 4, default: 200);
    total += _Generic(pointer, int *: 8, const int *: 100, default: 200);
    total += _Generic(const_pointer, const int *: 16, int *: 100, default: 200);
    total += _Generic(pair, struct Pair: 32, default: 200);
    total += _Generic("ok", char *: 64, default: 200);
    total += _Generic(bump(&calls), int: 1, default: bump(&calls));
    total += _Generic(scalar_only(), int: 1, default: 100);
    total += _Generic(pointer_only(), int *: 2, default: 100);
    total += _Generic(pair_only(), struct Pair: 4, default: 100);
    total += selected_values[0] + selected_values[1];
    total += identity(_Generic(value, int: 4, default: 100));
    total += (calls += 0, _Generic(value, int: 8, default: 100));
    total += value ? _Generic(letter, char: 16, default: 100) : 100;
    total += (_Generic(value, int: 32, default: 100));
    total += _Generic(pair, struct Pair: pair, default: pair).x;
    total += *selected_pointer + selected_pair.x;

    return total + calls;
}
