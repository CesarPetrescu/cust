enum Status { READY = 1, BUSY, DONE = 5 };
typedef enum Status Status;

Status global_status = READY;

char classify(Status status) {
    if (status == DONE) {
        return 9;
    }
    return 3;
}

Status next_status(Status status) {
    if (status == READY) {
        return BUSY;
    }
    return DONE;
}

int main() {
    Status current = global_status;
    current = next_status(current);

    Status values[2];
    values[0] = current;
    values[1] = DONE;

    int total = values[0] + values[1] + classify(current);
    {
        typedef char Status;
        Status marker = 'A';
        total += marker == 'A';
    }

    return total;
}
