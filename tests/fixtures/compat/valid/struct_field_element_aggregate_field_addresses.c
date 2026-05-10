struct Point {
    int x;
    int y;
};

struct Segment {
    struct Point start;
    struct Point end;
};

struct Drawing {
    struct Segment segments[2];
    int scale;
};

int move_start(struct Segment *segment, int dx, int dy) {
    segment->start.x = segment->start.x + dx;
    segment->start.y = segment->start.y + dy;
    return segment->start.x + segment->start.y;
}

int main(void) {
    struct Drawing drawing = {{{{1, 2}, {3, 4}}, {{5, 6}, {7, 8}}}, 9};
    struct Segment *second = drawing.segments + 1;
    struct Point *start = &second->start;
    start->x = start->x + 10;
    start->y = start->y + 20;

    struct Segment *first = drawing.segments + 0;
    int total = move_start(first, 30, 40);

    return total + drawing.segments[0].start.x + drawing.segments[0].start.y + drawing.segments[1].start.x + drawing.segments[1].start.y;
}
