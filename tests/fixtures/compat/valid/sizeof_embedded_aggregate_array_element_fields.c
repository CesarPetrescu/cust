struct Point {
    char tag;
    int x;
};

struct Segment {
    struct Point points[2];
};

int main(void) {
    struct Segment line = {{{1, 20}, {2, 30}}};
    int marker = 0;
    int ok = 0;

    ok += sizeof(line.points[0].tag) == sizeof(char);
    ok += sizeof(line.points[1].x) == sizeof(int);
    ok += sizeof((marker = marker + 1, line.points[0].tag)) == sizeof(char);
    ok += marker == 0;

    return ok;
}
