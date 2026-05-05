int global_int;
char global_char;
int numbers[3];
int *global_pointer;

int read_global_pointer() {
    return global_pointer == 0;
}

int main() {
    int local_int;
    char local_char;
    int *local_pointer;

    if (local_int != 0) {
        return 1;
    }
    if (local_char != 0) {
        return 2;
    }
    if (local_pointer != 0) {
        return 3;
    }
    if (global_int != 0 || global_char != 0) {
        return 4;
    }
    if (!read_global_pointer()) {
        return 5;
    }

    local_int = 7;
    local_pointer = &local_int;
    numbers[1] = *local_pointer;
    return numbers[0] + numbers[1] + numbers[2];
}
