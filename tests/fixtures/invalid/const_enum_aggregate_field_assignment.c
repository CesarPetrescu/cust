enum State {
    READY = 3,
    BUSY = 5
};

struct Job {
    const enum State fixed;
};

int main(void) {
    struct Job job = {READY};
    job.fixed = BUSY;
    return job.fixed;
}
