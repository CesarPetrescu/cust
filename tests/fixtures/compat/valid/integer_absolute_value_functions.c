int abs(int value);
long int labs(long int value);
long long int llabs(long long int value);

int mark(int *counter, int value) {
    *counter += 1;
    return value;
}

int main(void) {
    int counter = 0;

    if (abs(mark(&counter, -7)) != 7) {
        return 1;
    }
    if (labs(mark(&counter, -11)) != 11) {
        return 2;
    }
    if (llabs(mark(&counter, -13)) != 13) {
        return 3;
    }
    if (abs(5) != 5 || labs(0) != 0 || llabs(9) != 9) {
        return 4;
    }
    if (counter != 3) {
        return 5;
    }
    if (sizeof(abs(-1)) != sizeof(int)) {
        return 6;
    }

    return 0;
}
