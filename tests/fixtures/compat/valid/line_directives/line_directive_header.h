#line 700 "header-virtual.h"
int line_directive_header_probe(void) {
    char *file = __FILE__;
    int line = __LINE__;
    return text_equal(file, "header-virtual.h") && line == 702;
}
