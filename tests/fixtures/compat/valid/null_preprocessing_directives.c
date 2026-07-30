#
%:
# /**/
%: // comment-only null directive
#\
/* physically spliced null directive */
#if 1
#
#else
#error selected the wrong active branch
#endif
#if 0
# ignored inactive tokens
%:
#error inactive error
#endif
#include "null_preprocessing_directives/null_directive_header.h"

int main(void) {
    return null_directive_header_probe() ? 0 : 1;
}
