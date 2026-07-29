#define SOURCE_BONUS 3
#define OUTER_HEADER "include/outer.h"
#define SELECT_HEADER(value) value
#if 0
#include MALFORMED(
#endif
#include OUTER_HEADER
%:include SELECT_HEADER(OUTER_HEADER)
#include"include/outer.h"

int main(void) {
    return from_quoted_header(5) == 17 ? 0 : 1;
}
