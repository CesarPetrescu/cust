#define ENABLED

#ifdef ENABLED
#define OUTER 7
#else
this @ invalid C is skipped
#define OUTER 99
#endif

#ifndef DISABLED
#define INNER 5
#ifdef ENABLED
#define NESTED 3
#else
#define NESTED 90
#endif
#else
#define INNER 90
#endif

#ifdef DISABLED
#include <this_header_does_not_exist.h>
#define INACTIVE_DEFINED 1
#else
#define FALLBACK 2
#endif

#ifdef INACTIVE_DEFINED
#define LEAKED 1
#else
#define LEAKED 0
#endif

#undef ENABLED
#ifdef ENABLED
#define AFTER_UNDEF 100
#else
#define AFTER_UNDEF 4
#endif

int main(void) {
    return OUTER + INNER + NESTED + FALLBACK + AFTER_UNDEF == 21 && LEAKED == 0 ? 0 : 1;
}
