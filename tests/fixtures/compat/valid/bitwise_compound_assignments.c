int main() {
    int result = 0;

    int x = 12;
    result += x &= 10;
    result += x |= 3;
    result += x ^= 6;
    result += x <<= 2;
    result += x >>= 3;

    int y = 1;
    int z = 2;
    result += y <<= z += 1;

    int values[2];
    values[0] = 7;
    values[1] = 2;
    result += values[0] &= 3;
    result += values[1] <<= 2;

    int *p = values;
    result += p[0] |= 4;
    result += *p ^= 2;
    result += *(&values[1]) >>= 1;

    return result + (x &= 7);
}
