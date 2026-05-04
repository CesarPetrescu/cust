int factorial(int n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

int is_even(int n) {
    if (n == 0) {
        return 1;
    } else {
        return is_odd(n - 1);
    }
}

int is_odd(int n) {
    if (n == 0) {
        return 0;
    } else {
        return is_even(n - 1);
    }
}

int main() {
    return factorial(5) + is_even(8) + is_odd(7) * 4;
}
