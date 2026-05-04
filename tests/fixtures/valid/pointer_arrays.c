int sum3(int *values) {
    values[1] = values[1] + 5;
    return values[0] + values[1] + values[2];
}

int main() {
    int values[3];
    values[0] = 1;
    values[1] = 2;
    values[2] = 3;
    return sum3(values);
}
