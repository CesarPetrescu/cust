struct Point {
    int x;
    int y;
};

int main(void) {
    int marker = 0;
    int values[4] = {3, 5, 7, 11};
    int total = values[marker++, 2];

    values[marker++, 1] = 13;
    total += values[1];

    char text[4] = "cat";
    total += text[marker++, 1] == 'a';

    struct Point points[3] = {{1, 2}, {3, 4}, {5, 6}};
    total += points[marker++, 2].y;

    struct Point *cursor = points;
    total += cursor[marker++, 1].x;

    total += marker == 5;
    return total;
}
