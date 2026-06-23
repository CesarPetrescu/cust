struct Box {
    struct { int x; int y; } point;
    union { int value; char tag; } number;
    struct { int value; } items[2];
};

int main(void) {
    struct Box box = {{2, 3}, {5}, {{7}, {11}}};
    box.point.x += 4;
    box.items[1].value = box.point.y + box.number.value;
    return box.point.x + box.items[0].value + box.items[1].value;
}
