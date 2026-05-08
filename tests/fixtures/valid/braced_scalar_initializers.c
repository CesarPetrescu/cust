int side_effect(int *slot) {
    *slot += 1;
    return *slot + 4;
}

struct Pair {
    int left;
    char right;
    int values[3];
};

union Number {
    int value;
    char tag;
};

int sum_array(int values[]) {
    return values[0] + values[1] + values[2];
}

int main(void) {
    int calls = 0;
    int scalar = {side_effect(&calls)};
    char letter = {'A'};
    int values[3] = {{1}, {2}, [2] = {3}};
    struct Pair pair = {{scalar}, {2}, {{7}, {8}, {9}}};
    struct Pair designated = {.left = {10}, .right = {3}, .values = {{4}, {5}, {6}}, .values[1] = {11}};
    union Number number = {{12}};
    return calls + scalar + letter - 65 + sum_array(values) + pair.left + pair.right + pair.values[2] + designated.left + designated.right + designated.values[1] + designated.values[2] + number.value;
}
