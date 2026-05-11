#include <stddef.h>

_Static_assert(1, "top level literal true");
_Static_assert(sizeof(char) == 1, "top level sizeof true");

int main(void) {
    _Static_assert(2 + 3 == 5, "block expression true");
    _Static_assert(sizeof(int[2]) == sizeof(int) * 2, "array type size true");
    return 17;
}
