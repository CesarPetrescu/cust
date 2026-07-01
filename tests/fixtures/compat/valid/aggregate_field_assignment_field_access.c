struct Point {
    int x;
    char tag;
};

struct Line {
    struct Point start;
    struct Point end;
};

int main(void) {
    struct Point replacement = {7, 4};
    struct Point other = {5, 6};
    struct Line line = {{1, 2}, {3, 4}};
    struct Line *slot = &line;
    int marker = 0;
    int total = 0;

    total += (line.start = replacement).x;
    total += line.start.tag;
    total += (slot->end = other).tag;
    total += slot->end.x;
    total += sizeof((marker = marker + 1, (line.end = other).tag)) == sizeof(char);
    total += marker;

    return total;
}
