int marker = 0;

struct Point {
    int x;
    char tag;
};

union Number {
    int value;
    char tag;
};

struct Line {
    struct Point points[2];
    union Number numbers[3];
    int values[4];
};

struct Box {
    struct Line line;
};

struct Line make_line(int base) {
    marker = marker + 100;
    struct Line line = {
        {{base, 'a'}, {base + 1, 'b'}},
        {{base}, {base + 2}, {base + 3}},
        {1, 2, 3, 4}
    };
    return line;
}

struct Box make_box(int base) {
    marker = marker + 1000;
    struct Box box = {make_line(base)};
    return box;
}

int main(void) {
    struct Line line = {
        {{1, 'x'}, {2, 'y'}},
        {{3}, {4}, {5}},
        {6, 7, 8, 9}
    };
    int total = 0;

    total = total + sizeof(make_line(10).points) / sizeof(make_line(10).points[0]);
    total = total + sizeof((line = make_line(20)).numbers) / sizeof((line = make_line(20)).numbers[0]);
    total = total + sizeof((1 ? line : make_line(30)).values) / sizeof((1 ? line : make_line(30)).values[0]);
    total = total + sizeof(make_box(40).line.points) / sizeof(make_box(40).line.points[0]);

    return total + marker + line.points[0].x;
}
