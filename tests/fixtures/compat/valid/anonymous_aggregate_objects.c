struct { int left; int right; } global_pair = {1, 2};

int main(void) {
    struct { int x; int y; } point = {3, 4};
    union { int value; char tag; } number = {5};
    struct { int values[2]; } packet = {{6, 7}};
    struct { int x; int y; } points[2] = {{8, 9}, {.y = 10, .x = 11}};
    return global_pair.left + global_pair.right + point.x + point.y + number.value + packet.values[1] + points[0].y + points[1].x;
}
