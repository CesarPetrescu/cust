struct Point { int x; int y; };
struct Box { const struct Point points[2]; };

int main(void) {
    ((struct Box){{{1, 2}, {3, 4}}}).points[1].x = 9;
    return 0;
}
