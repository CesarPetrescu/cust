int main() {
    int values[2];
    const int *p = values;
    p[0] = 3;
    return values[0];
}
