struct Anchor {
    int x;
    int y;
};

int main(void) {
    struct Anchor *points = (struct Anchor[]){
        { sizeof(struct AggLitBox { int value; }) == sizeof(struct AggLitBox), 2 },
        [sizeof(struct AggLitIndex { char tag; }) == sizeof(struct AggLitIndex)] = {
            sizeof(union AggLitChoice { int value; char tag; }) == sizeof(union AggLitChoice),
            ((struct AggLitPoint { int x; }){5}).x
        }
    };
    struct AggLitBox box = {7};
    struct AggLitIndex index = {'A'};
    union AggLitChoice choice = {11};
    struct AggLitPoint point = {13};
    return points[0].x + points[0].y + points[1].x + points[1].y
        + box.value + index.tag + choice.value + point.x;
}
