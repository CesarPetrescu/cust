int rand(void);
void srand(unsigned int seed);

unsigned int marked_seed(int *calls, unsigned int seed) {
    *calls += 1;
    return seed;
}

int main(void) {
    int default_first = rand();
    int default_second = rand();
    int default_third = rand();

    srand(1);
    if (rand() != default_first || rand() != default_second || rand() != default_third) {
        return 1;
    }

    int seed_calls = 0;
    srand(marked_seed(&seed_calls, 73));
    int first = rand();
    int second = rand();
    int third = rand();
    int fourth = rand();

    srand(73);
    if (rand() != first || rand() != second || rand() != third || rand() != fourth) {
        return 2;
    }
    if (seed_calls != 1) {
        return 3;
    }

    return first >= 0 && second >= 0 && third >= 0 && fourth >= 0 ? 0 : 4;
}
