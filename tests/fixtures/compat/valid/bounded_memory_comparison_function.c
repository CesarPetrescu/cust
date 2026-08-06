int memcmp(const void *left, const void *right, unsigned long int count);

int left_calls = 0;
int right_calls = 0;
int count_calls = 0;
char called_left[3] = {'a', 'b', 'c'};
char called_right[3] = {'a', 'b', 'd'};

const void *left_once(void) {
    left_calls += 1;
    return called_left;
}

const void *right_once(void) {
    right_calls += 1;
    return called_right;
}

int count_once(void) {
    count_calls += 1;
    return 3;
}

int main(void) {
    char equal_left[4] = {'a', 0, 'b', 'c'};
    char equal_right[4] = {'a', 0, 'b', 'c'};
    if (memcmp(equal_left, equal_right, 4) != 0) {
        return 1;
    }

    char lower[4] = {'a', 0, 'b', 'c'};
    char higher[4] = {'a', 0, 'c', 'a'};
    if (memcmp(lower, higher, 4) >= 0 || memcmp(higher, lower, 4) <= 0) {
        return 2;
    }

    signed char high_byte[1] = {-1};
    signed char low_byte[1] = {1};
    if (memcmp(high_byte, low_byte, 1) <= 0 || memcmp(low_byte, high_byte, 1) >= 0) {
        return 3;
    }

    char offset_left[5] = {'x', 'a', 'b', 'c', 'z'};
    char offset_right[5] = {'y', 'a', 'b', 'c', 'w'};
    if (memcmp(offset_left + 1, offset_right + 1, 3) != 0) {
        return 4;
    }

    if (memcmp(offset_left + 4, offset_right + 4, 0) != 0) {
        return 5;
    }

    if (memcmp(left_once(), right_once(), count_once()) >= 0 ||
        left_calls != 1 || right_calls != 1 || count_calls != 1) {
        return 6;
    }

    return 0;
}
