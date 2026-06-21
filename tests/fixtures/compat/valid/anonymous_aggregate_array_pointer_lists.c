int main(void) {
    struct { int x; int y; } points[3] = {{1, 2}, {.x = 3, .y = 4}, {5, 6}}, *slot = points + 1;
    union { int value; char tag; } numbers[2] = {{7}, {.tag = 8}}, *picked = numbers;
    slot->y = slot->y + points[2].x;
    picked[1].value = picked[1].value + slot->x;
    return points[0].x + points[1].y + points[1].x + numbers[0].value + numbers[1].value;
}
