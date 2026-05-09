struct Point {
    int x;
    int y;
};

struct Box {
    struct Point point;
    int value;
};

int bump(void) {
    static int calls = 0;
    calls++;
    return calls;
}

int main(void) {
    int total = 0;
    total += (((struct Point){1, 2}).x = 7);
    total += (((struct Point){3, 4}).y += 5);
    total += ++((struct Point){5, 6}).x;
    total += ((struct Point){7, 8}).y++;
    total += (((struct Box){{1, 2}, 3}).point.x = 4);
    total += (((struct Point){bump(), 2}).x = 10);
    total += bump();
    return total;
}
