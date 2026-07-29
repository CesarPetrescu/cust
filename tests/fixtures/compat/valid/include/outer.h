#ifndef CUST_OUTER_H
#define CUST_OUTER_H
#define NESTED_HEADER "nested.h"
#define COMMON_HEADER "../common.h"
#define FORWARD_HEADER(value) value
#include FORWARD_HEADER(NESTED_HEADER)
#include COMMON_HEADER
#define HEADER_VALUE (NESTED_ADD(SOURCE_BONUS) + COMMON_VALUE)
int from_quoted_header(int value) {
    return value + HEADER_VALUE;
}
#endif
