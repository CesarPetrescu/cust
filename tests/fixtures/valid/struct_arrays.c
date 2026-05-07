struct Point {
    int x;
    int y;
};

struct Packet {
    int values[2];
    int weight;
};

int sum_point(struct Point p) {
    p.x += 10;
    return p.x + p.y;
}

int main() {
    struct Point points[3] = {{1, 2}, {3, 4}};
    struct Packet packets[2] = {{{5, 6}, 7}, {{8}, 9}};

    int total = points[0].x + points[0].y + points[1].x + points[1].y;
    total += points[2].x + points[2].y;

    points[2].x = 10;
    points[2].y = points[0].x + points[1].y;
    points[1].x += 5;
    points[0].y++;

    struct Point copy;
    copy = points[1];
    copy.y = 20;
    total += points[1].x + points[1].y + copy.x + copy.y;
    total += sum_point(points[0]);
    total += points[0].x + points[0].y;

    packets[1].values[1] = 11;
    total += packets[0].values[0] + packets[0].values[1] + packets[0].weight;
    total += packets[1].values[0] + packets[1].values[1] + packets[1].weight;

    total += sizeof(points) / sizeof(points[0]);
    total += sizeof(points[0].x);

    return total;
}
