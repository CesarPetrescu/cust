static inline int add_one(int value);

static inline int add_one(int value) {
    return value + 1;
}

_Noreturn void declared_only(void);

int main(void) {
    int value = add_one(4);
    return value + 6;
}
