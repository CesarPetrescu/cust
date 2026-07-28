int PRESERVE(int value) {
    return value + 10;
}

#define ADD(left, right) ((left) + (right))
#define DOUBLE(value) ADD(value, value)
#define DECLARE(type, name, value) type name = value
#define ZERO() 0
#define BASE 4
#define APPLY(function, value) function(value)
#define ID(value) (value)
#define FORWARD(ignored) ID
#define PRESERVE(value) value

#if defined(ADD) && ADD(1, 2) == 3
#define CONDITION_RESULT 0
#else
#define CONDITION_RESULT 1
#endif

int main(void) {
    DECLARE(int, total, DOUBLE(BASE + 1));
    int nested = ADD((1 + 2), ADD(3, 4));
    return total == 10 && nested == 10 && APPLY(DOUBLE, 3) == 6 && ID(7) == 7
        && FORWARD(0)(9) == 9 && PRESERVE(PRESERVE)(1) == 11
        ? ZERO() + CONDITION_RESULT
        : 1;
}
