int sum3(int values[3]) {
    return values[0] + values[1] + values[2];
}

int fib(int n) {
    if (n <= 1) {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

int first_and_last(char text[4]) {
    return text[0] + text[2] + text[3];
}

int main() {
    int values[3];
    values[0] = 7;
    values[1] = 11;
    values[2] = 13;

    return sum3(values) + fib(6) + first_and_last("Az!");
}
