int main(void) {
    int total = 0;

    for (struct { int x; } point = {1}; point.x < 4; point.x++) {
        total += point.x;
    }

    for (union { int value; char tag; } number = {5}; number.value < 7; number.value++) {
        total += number.value;
    }

    return total;
}
