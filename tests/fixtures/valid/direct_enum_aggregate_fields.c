enum State {
    READY = 3,
    BUSY = 5
};

struct Job {
    enum State state;
    const enum State fixed;
    enum State states[3];
    enum State *cursor;
};

int score(struct Job *job, const enum State values[static 3]) {
    int total = job->fixed;
    if (job->state == READY) {
        total = total + 1;
    }
    total = total + job->states[1];
    job->states[2] = READY;
    total = total + job->cursor[1];
    enum State *middle = &job->cursor[1];
    total = total + *middle;
    total = total + values[2];
    return total;
}

int main(void) {
    enum State values[3] = {READY, BUSY, 7};
    struct Job job = {READY, BUSY, {1, BUSY, 9}, values};
    enum State *direct_middle = &job.cursor[1];
    return score(&job, values) + job.states[2] + job.cursor[0] + *direct_middle;
}
