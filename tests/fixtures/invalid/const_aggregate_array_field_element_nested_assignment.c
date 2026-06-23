struct Point { int x; int y; };
struct Box { const struct Point points[2]; };

int main(void) {
    struct Box box = {{{1, 2}, {3, 4}}};
    box.points[1].x = 9;
    return box.points[1].x;
}
