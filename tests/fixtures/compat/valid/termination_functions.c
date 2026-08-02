_Noreturn void exit(int status);
_Noreturn void _Exit(int status);
_Noreturn void abort(void);

int marked_status(int *calls, int base) {
    *calls += 1;
    return base + *calls;
}

int main(void) {
    int calls = 0;
    if (calls == 1) {
        exit(20);
    }
    if (calls == 2) {
        abort();
    }
    _Exit(marked_status(&calls, 22));
}
