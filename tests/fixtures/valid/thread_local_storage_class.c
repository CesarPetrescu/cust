_Thread_local int global_counter = 2;
static _Thread_local int hidden_counter = 3;

int bump(void) {
    static _Thread_local int local_counter = 4;
    local_counter = local_counter + 1;
    return local_counter;
}

int main(void) {
    global_counter = global_counter + 1;
    hidden_counter = hidden_counter + bump();
    return global_counter + hidden_counter + bump();
}
