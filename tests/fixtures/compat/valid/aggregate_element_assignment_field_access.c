struct Point {
    int x;
    char tag;
};

struct Line {
    struct Point points[2];
};

int main(void) {
    int marker = 0;
    struct Point points[2] = {{1, 'a'}, {2, 'b'}};
    struct Point replacement = {5, 'z'};
    struct Line line = {{{3, 'c'}, {4, 'd'}}};
    struct Line *line_ptr = &line;
    int total = 0;

    total += (points[0] = replacement).x == 5;
    total += points[0].tag == 'z';
    total += (line.points[1] = replacement).tag == 'z';
    total += line.points[1].x == 5;
    total += (line_ptr->points[0] = replacement).x == 5;
    total += line.points[0].tag == 'z';

    total += sizeof((marker = marker + 1, (points[1] = replacement).tag)) == sizeof(char);
    total += sizeof((marker = marker + 10, (line.points[0] = replacement).x)) == sizeof(int);
    total += marker == 0;
    total += points[1].tag == 'b';
    total += line.points[0].x == 5;

    return total;
}
