#define VERSION 3
#define SCALE 2
#define MAX_UNSIGNED 0xffffffffffffffffULL

#if (MAX_UNSIGNED >> 63) == 1 && (0U - 1U) > 0U
#define UNSIGNED_CHECK 0
#else
#define UNSIGNED_CHECK 1
#endif

#if VERSION == 1
#define RESULT 10
#elif defined(VERSION) && VERSION * SCALE == 6
#define RESULT 7
#elif 1 / 0
#define RESULT 20
#else
#define RESULT 30
#endif

#if 0
#define SKIPPED 1
#elif defined RESULT && !defined(SKIPPED)
#define EXTRA ((1 << 3) | 2)
#else
#define EXTRA 90
#endif

#if UNKNOWN_IDENTIFIER
#define SELECTED 40
#elif (RESULT == 7) ? 1 : 0
#define SELECTED 4
#else
#define SELECTED 50
#endif

int main(void) {
    return UNSIGNED_CHECK + RESULT + EXTRA + SELECTED == 21 ? 0 : 1;
}
