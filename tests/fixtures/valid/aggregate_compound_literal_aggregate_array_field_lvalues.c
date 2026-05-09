struct Point {
    int x;
    int y;
};

struct Line {
    struct Point points[2];
};

int adjust(struct Point *point) {
    point->y = point->y + 5;
    return point->x + point->y;
}

int main(void) {
    int total = (((struct Line){{{1, 2}, {3, 4}}}).points[0].x = 7);
    total = total + (((struct Line){{{2, 3}, {4, 5}}}).points[1].y += 6);
    total = total + ++((struct Line){{{3, 4}, {5, 6}}}).points[1].x;
    total = total + ((struct Line){{{4, 5}, {6, 7}}}).points[0].y++;
    total = total + adjust(&((struct Line){{{8, 9}, {10, 11}}}).points[1]);
    return total;
}
