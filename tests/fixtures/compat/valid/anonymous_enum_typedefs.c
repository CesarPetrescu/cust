typedef enum { MODE_A = 2, MODE_B, MODE_C = 7 } Mode;

Mode global_mode = MODE_A;

Mode choose_mode(Mode left, Mode right, int pick_right) {
    if (pick_right) {
        return right;
    }
    return left;
}

int score(Mode mode) {
    if (mode == MODE_C) {
        return 11;
    }
    return 5;
}

int main(void) {
    Mode current = choose_mode(global_mode, MODE_C, 1);
    int total = current + score(current);

    {
        typedef enum { MODE_A = 3, BLOCK_EXTRA = 4 } BlockMode;
        BlockMode local = MODE_A;
        total += local + BLOCK_EXTRA;
    }

    total += MODE_B;
    return total;
}
