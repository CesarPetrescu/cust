#ifndef PREDEFINED_FILE_LINE_HEADER_H
#define PREDEFINED_FILE_LINE_HEADER_H
#define HEADER_FILE __FILE__
#define HEADER_LINE __LINE__
int header_predefined_probe(void) {
    char *file = HEADER_FILE;
    int line = HEADER_LINE;
    return text_ends_with(file, "predefined/predefined_file_line_header.h") && line == 7;
}
#endif
