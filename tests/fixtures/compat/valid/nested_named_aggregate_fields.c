struct Scene {
    struct Point { int x; int y; } origin, cursor;
    union Number { int value; char tag; } primary, secondary;
    struct Segment { struct Point start; struct Point end; } segments[2];
};

int main(void) {
    struct Scene scene = {
        {1, 2},
        {3, 4},
        {5},
        {6},
        {{{7, 8}, {9, 10}}, {{11, 12}, {13, 14}}}
    };
    scene.cursor.x = scene.origin.x;
    scene.cursor.y = scene.origin.y;
    scene.cursor.x += 2;
    scene.cursor.y += 3;
    scene.segments[1].end.x = scene.cursor.x + scene.cursor.y;
    scene.secondary.tag = 4;
    return scene.cursor.x
        + scene.cursor.y
        + scene.primary.value
        + scene.secondary.value
        + scene.segments[0].start.x
        + scene.segments[0].start.y
        + scene.segments[0].end.x
        + scene.segments[0].end.y
        + scene.segments[1].end.x;
}
