int bump(int *p) {
    return (*p += 1, *p += 2, *p);
}

int combine(int left, int right) {
    return left * 10 + right;
}

int main() {
    int x = 1;
    int y = 0;
    int i = 0;
    int limit = 0;
    int lhs_effect = 0;
    int call_value = 0;
    int bumped = 0;
    int values[3];
    int *p = 0;

    x = (y = 3, y += 4, y);
    lhs_effect = (x += 1, y += x, x + y);

    for ((i = 0, values[0] = 1); (limit = 3, i < limit); (i++, values[i - 1] += i)) {
        values[i] = i + 2;
    }

    if (p = &x, p) {
        *p += 5;
    } else {
        return 99;
    }

    call_value = combine((x += 1, x), (y += 1, y));
    bumped = bump(&values[0]);

    return x + y + values[0] + values[1] + values[2] + lhs_effect + call_value + bumped;
}
