struct Point {
    int x;
    int y;
};

struct Segment {
    struct Point points[2];
};

int main(void) {
    struct Segment segment = {.points[2].x = 7};
    return segment.points[0].x;
}
