struct Point {
    int x;
    char y;
};

struct Line {
    struct Point points[2];
};

union Number {
    int value;
    char tag;
};

struct Bag {
    union Number numbers[2];
};

int main(void) {
    struct Point points[2] = {{1, 2}, {3, 4}};
    struct Point replacement = {5, 6};
    struct Point *slot = points;
    struct Line line = {{{7, 8}, {9, 10}}};
    struct Line *line_ptr = &line;
    struct Bag bag = {{{11}, {12}}};
    struct Bag *bag_ptr = &bag;
    union Number number_replacement = {13};
    int marker = 0;
    int ok = 0;

    ok += sizeof((points[0] = replacement)) == sizeof(struct Point);
    ok += points[0].x == 1;
    ok += sizeof((*slot = replacement)) == sizeof(struct Point);
    ok += points[0].x == 1;
    ok += sizeof((line.points[1] = replacement)) == sizeof(struct Point);
    ok += line.points[1].x == 9;
    ok += sizeof((line_ptr->points[0] = replacement)) == sizeof(struct Point);
    ok += line.points[0].x == 7;
    ok += sizeof((bag_ptr->numbers[1] = number_replacement)) == sizeof(union Number);
    ok += bag.numbers[1].value == 12;
    ok += sizeof((marker = marker + 1, points[1] = replacement)) == sizeof(struct Point);
    ok += marker == 0;

    return ok;
}
