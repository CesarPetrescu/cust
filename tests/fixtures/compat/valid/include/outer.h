#ifndef CUST_OUTER_H
#define CUST_OUTER_H
#include "nested.h"
#include "../common.h"
#define HEADER_VALUE (NESTED_ADD(SOURCE_BONUS) + COMMON_VALUE)
int from_quoted_header(int value) {
    return value + HEADER_VALUE;
}
#endif
