_Static_assert(1, "top level literal true");
_Static_assert(sizeof(char) == 1, "top level sizeof true");
_Static_assert(sizeof(enum TopLevelAssertEnum { TOP_ASSERT_VALUE = 13 }) == sizeof(int), "top level inline enum type true");

int main(void) {
    _Static_assert(2 + 3 == 5, "block expression true");
    _Static_assert(sizeof(int[2]) == sizeof(int) * 2, "array type size true");
    _Static_assert(sizeof(enum BlockAssertEnum { BLOCK_ASSERT_VALUE = 19 }) == sizeof(int), "block inline enum type true");
    return TOP_ASSERT_VALUE + BLOCK_ASSERT_VALUE;
}
