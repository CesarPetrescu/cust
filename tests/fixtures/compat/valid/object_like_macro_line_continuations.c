#define BASE 1 \
    + 2
#define RESULT BASE \
    + 4

#if RESULT == \
    7
#define SELECTED RESULT
#elif RESULT == 0
#define SELECTED 99
#else
#define SELECTED 88
#endif

int main(void) {
    return SELECTED - 7;
}
