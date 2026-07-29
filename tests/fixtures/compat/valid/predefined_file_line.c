int text_ends_with(char *text, char *suffix) {
    int text_length = 0;
    int suffix_length = 0;
    while (text[text_length] != '\0') { text_length++; }
    while (suffix[suffix_length] != '\0') { suffix_length++; }
    if (suffix_length > text_length) { return 0; }
    for (int i = 0; i < suffix_length; i++) {
        if (text[text_length - suffix_length + i] != suffix[i]) { return 0; }
    }
    return 1;
}
#include "predefined/predefined_file_line_header.h"
#if __LINE__ != 13
@
#endif
int main(void) {
    char *file = __FILE__;
    int line = __LINE__;
    return text_ends_with(file, "predefined_file_line.c") && line == 18
            && header_predefined_probe()
        ? 0
        : 1;
}
