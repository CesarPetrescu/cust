struct Pair {
    int x;
    int y;
};

struct Box {
    struct Pair point;
    int values[3];
};

int main(void) {
    struct Box box = {
        .point = {
            .x = sizeof(enum InitX { INIT_X = 4 }) ? INIT_X : 0,
            .y = (enum InitY { INIT_Y = 7 })0 + INIT_Y,
        },
        .values = {
            [sizeof(enum SlotIndex { SLOT_INDEX = 1 }) ? SLOT_INDEX : 0] = SLOT_INDEX + 10,
            [2] = _Alignof(enum TailValue { TAIL_VALUE = 6 }) ? TAIL_VALUE : 0,
        },
    };

    return box.point.x * 20 + box.point.y + box.values[1] + box.values[2] + INIT_X + INIT_Y + SLOT_INDEX + TAIL_VALUE;
}
