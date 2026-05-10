_Bool global_flag = 1;
const _Bool const_flag = 0;
static _Bool static_flag = 1;

_Bool keep(_Bool value);

_Bool keep(_Bool value) {
    return value;
}

int takes_bool(_Bool flag, _Bool values[2], _Bool *slot) {
    _Bool local = flag;
    _Bool from_array = values[1];
    _Bool *cursor = slot;
    *cursor = keep(from_array);
    return local + *cursor + (sizeof(_Bool) == sizeof(const _Bool)) + (sizeof(_Bool[2]) == sizeof(_Bool) * 2);
}

int main(void) {
    typedef _Bool Flag;
    Flag alias_flag = keep(global_flag);
    _Bool values[2] = {0, 1};
    _Bool slot = 0;
    int iterated = 0;
    for (_Bool i = 0; i < 1; i = i + 1) {
        slot = alias_flag;
        iterated = iterated + 1;
    }
    return takes_bool(alias_flag, values, &slot) + static_flag + const_flag + iterated;
}
