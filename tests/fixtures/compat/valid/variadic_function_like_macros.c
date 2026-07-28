#define BASE 4
#define CALL(function, ...) function(__VA_ARGS__)
#define FORWARD(...) CALL(sum3, __VA_ARGS__)
#define PICK(first, ...) first
#define EMPTY(...) 0
#define NEEDS_TWO(first, second) ((first) + (second))
#define OPTIONAL(...) (0 __VA_ARGS__)
#define OMIT(first, ...) ((first) __VA_ARGS__)
#define SUM2(first, ...) ((first) + (__VA_ARGS__))

#if SUM2(1, 2) == 3
#define CONDITION_RESULT 0
#else
#define CONDITION_RESULT 1
#endif

int sum3(int first, int second, int third) {
    return first + second + third;
}

int main(void) {
    int direct = CALL(sum3, BASE, 2, 3);
    int forwarded = FORWARD(1, (1 + 1), 3);
    return direct == 9 && forwarded == 6 && PICK(7, 8, 9) == 7
            && EMPTY() == 0 && EMPTY(NEEDS_TWO(1)) == 0
            && OPTIONAL() == 0 && OPTIONAL(+ 2) == 2
            && OMIT(3,) == 3
        ? CONDITION_RESULT
        : 1;
}
