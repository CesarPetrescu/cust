enum Status { STARTED = 2, RUNNING, DONE = 7 };

int score() {
    enum Local { LOW = -1, MID, HIGH = 4 };
    int total = STARTED + RUNNING + DONE + LOW + MID + HIGH;
    {
        enum { RUNNING = 10 };
        total += RUNNING;
    }
    total += RUNNING;
    return total;
}

int main() {
    enum { EXTRA = 5 };
    int value = score();
    if (DONE) {
        value += EXTRA;
    }
    return value;
}
