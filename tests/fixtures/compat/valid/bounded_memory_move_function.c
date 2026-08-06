void *memmove(void *destination,
              const void *source,
              unsigned long int count);

int main(void) {
    char forward[7] = {'a', 'b', 'c', 'd', 'e', 'f', 0};
    void *forward_result = memmove(forward + 2, forward, 4);
    if (forward_result != forward + 2 ||
        forward[0] != 'a' || forward[1] != 'b' ||
        forward[2] != 'a' || forward[3] != 'b' ||
        forward[4] != 'c' || forward[5] != 'd' || forward[6] != 0) {
        return 1;
    }

    char backward[7] = {'a', 'b', 'c', 'd', 'e', 'f', 0};
    void *backward_result = memmove(backward, backward + 2, 4);
    if (backward_result != backward ||
        backward[0] != 'c' || backward[1] != 'd' ||
        backward[2] != 'e' || backward[3] != 'f' ||
        backward[4] != 'e' || backward[5] != 'f' || backward[6] != 0) {
        return 2;
    }

    char same[3] = {'x', 'y', 0};
    if (memmove(same, same, 3) != same ||
        same[0] != 'x' || same[1] != 'y' || same[2] != 0) {
        return 3;
    }

    signed char negative[4] = {-1, -2, -3, -4};
    memmove(negative + 1, negative, 3);
    if (negative[0] != -1 || negative[1] != -1 ||
        negative[2] != -2 || negative[3] != -3) {
        return 4;
    }

    return 0;
}