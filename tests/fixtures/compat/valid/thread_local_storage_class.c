_Thread_local int global_counter = 2;
static _Thread_local int hidden_counter = 3;
_Thread_local const struct { int bonus; } global_shape = {5};

int bump(void) {
    static _Thread_local int local_counter = 4;
    static _Thread_local const struct { int step; } local_shape = {2};
    local_counter = local_counter + 1;
    return local_counter + local_shape.step;
}

int main(void) {
    global_counter = global_counter + 1;
    hidden_counter = hidden_counter + bump();
    return global_counter + hidden_counter + global_shape.bonus + bump();
}
