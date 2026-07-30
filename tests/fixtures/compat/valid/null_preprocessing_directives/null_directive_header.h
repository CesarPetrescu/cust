#ifndef NULL_DIRECTIVE_HEADER_H
#define NULL_DIRECTIVE_HEADER_H

#
%: /* header digraph */
#if 0
# ignored inactive header tokens
%:
#error inactive header error
#endif

int null_directive_header_probe(void) {
    return 1;
}

#endif
