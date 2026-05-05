int mix(char *text, int *values) {
    int total = 0;
    for (int i = 0; i < 3; i = i + 1) {
        total = total + text[i];
    }

    values[1] = values[1] + text[1] - 'A';
    int *tail = &values[1];
    tail[1] = tail[0] + text[2] - 'A';

    return total + values[0] + values[1] + values[2] + text[3];
}

int main() {
    int values[3];
    values[0] = 3;
    values[1] = 4;
    values[2] = 5;

    return mix("ABC", values) - values[2] * 10;
}
