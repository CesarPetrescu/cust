int text_equal(char *left, char *right) {
    while (*left == *right && *left != '\0') { left++; right++; }
    return *left == *right;
}
#line 50 "outer-virtual.c"
#include "line_directives/line_directive_header.h"
int main(void) {
    char *file = __FILE__;
    int line = __LINE__;
    return text_equal(file, "outer-virtual.c") && line == 53
            && line_directive_header_probe()
        ? 0
        : 1;
}
