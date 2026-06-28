enum { BASE_INDEX = 1, SLOT_INDEX = BASE_INDEX + 1 };

int global_values[4] = {
    [SLOT_INDEX] = 5,
    [sizeof(char)] = 7,
};

struct Point {
    int x;
    int y;
};

struct Packet {
    int values[4];
    struct Point points[3];
};

int main(void) {
    int local_values[5] = {
        [BASE_INDEX + 2] = 11,
        [sizeof "hi" - 1] = 13,
    };
    int *fixed_literal = (int[4]){
        [SLOT_INDEX ? 3 : 0] = 17,
        [BASE_INDEX] = 19,
    };
    int *inferred_literal = (int[]){
        [BASE_INDEX + 2] = 7,
    };
    struct Packet packet = {
        .values = {[SLOT_INDEX] = 23},
        .points = {[BASE_INDEX + 1] = {.x = 29, .y = 31}},
    };
    struct Point points[3] = {
        [sizeof(char)] = {37, 41},
    };

    return global_values[1] + global_values[2] +
           local_values[2] + local_values[3] +
           fixed_literal[1] + fixed_literal[3] +
           inferred_literal[3] + packet.values[2] +
           packet.points[2].x + packet.points[2].y +
           points[1].x + points[1].y;
}
