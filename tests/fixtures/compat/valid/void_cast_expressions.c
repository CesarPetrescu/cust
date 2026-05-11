int marker = 0;

int bump(int value) {
    marker = marker + value;
    return value * 2;
}

void touch(int value) {
    marker = marker + value;
}

int main(void) {
    int values[3] = {1, 2, 3};
    int *cursor = values + 1;

    (void)bump(5);
    (void)*cursor;
    (void)(cursor + 1);
    (void)touch(7);
    (void)(marker = marker + values[2]);

    return marker;
}
