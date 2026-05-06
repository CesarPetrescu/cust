int sum_from_second(int *p) {
    int total = 0;
    int *cursor = p + 1;
    total += *cursor;
    cursor++;
    total += *cursor;
    total += cursor - p;
    --cursor;
    total += *cursor;
    cursor += 2;
    total += cursor[-1];
    cursor -= 3;
    total += *cursor;
    return total;
}

int main() {
    int values[4];
    values[0] = 5;
    values[1] = 7;
    values[2] = 11;
    values[3] = 13;

    int *base = values;
    int *third = 2 + base;
    char *text = "abc";
    int result = sum_from_second(base);
    result += *third;
    result += third - base;
    result += *(base + 3);
    result += *(text + 1) - 'a';
    return result;
}
