int global_int;
char global_char;
int numbers[3];

int main() {
    if (global_int != 0) {
        return 1;
    }
    if (global_char != 0) {
        return 2;
    }
    return numbers[0] + 7;
}
