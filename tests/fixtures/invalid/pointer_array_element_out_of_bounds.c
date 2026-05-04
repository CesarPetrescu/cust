int main() {
    int values[2];
    values[0] = 4;
    values[1] = 5;
    int *last = &values[1];
    return last[1];
}
