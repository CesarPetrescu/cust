int sum3(int values[3]) {
    return values[0] + values[1] + values[2];
}

int main() {
    int values[3];
    values[0] = 10;
    values[1] = 20;
    values[2] = values[0] + values[1];

    char letters[2];
    letters[0] = 'A';
    letters[1] = '\n';

    int i = 0;
    int total = 0;
    while (i < 3) {
        total = total + values[i];
        i = i + 1;
    }

    values[0];
    return total + sum3(values) + letters[0] + letters[1];
}
