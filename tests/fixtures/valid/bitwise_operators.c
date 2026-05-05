int main() {
    int x = 42;
    int y = 12;
    int result = 0;

    result += x & y;
    result += x | y;
    result += x ^ y;
    result += ~0 + 2;
    result += 1 << 5;
    result += 64 >> 3;
    result += 1 + 2 << 3;
    result += 32 >> 1 + 2;
    result += 7 & 3 == 3;
    result += (7 & 3) == 3;
    result += 4 | 1 && 0 ? 100 : 5;

    int values[2];
    values[0] = 5;
    values[1] = 10;
    result += values[0] << 1;
    int *p = values;
    result += p[1] & 14;

    return result;
}
