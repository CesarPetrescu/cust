struct Frame {
    int matrix[2][3];
    char chars[2][2];
};

union Overlay {
    int matrix[2][2];
    int values[4];
};

int main(void) {
    struct Frame frame = {{{1, 2, 3}, {4, 5, 6}}, {{'a', 'b'}, {'c', 'd'}}};
    struct Frame copy = frame;
    struct Frame frames[2] = {
        {{{7, 8, 9}, {10, 11, 12}}, {{'e', 'f'}, {'g', 'h'}}},
        {{{13, 14, 15}, {16, 17, 18}}, {{'i', 'j'}, {'k', 'l'}}}
    };
    struct Frame *slot = &frames[1];
    union Overlay overlay = {.matrix = {{19, 20}, {21, 22}}};
    struct {
        int grid[2][2];
    } anonymous = {{{23, 24}, {25, 26}}};
    struct Frame partial = {{{27}, {28}}, {{'m'}, {'n'}}};

    copy.matrix[0][0] = 30;
    frames[0].matrix[1][2] += 2;
    slot->matrix[0][1]++;
    anonymous.grid[1][0] = 31;

    if (frame.matrix[0][0] != 1 || copy.matrix[0][0] != 30) {
        return 1;
    }
    if (frames[0].matrix[1][2] != 14 || frames[1].matrix[0][1] != 15) {
        return 2;
    }
    if (slot->matrix[0][1] != 15 || slot->chars[1][0] != 'k') {
        return 3;
    }
    if (overlay.matrix[1][1] != 22 || anonymous.grid[1][0] != 31) {
        return 4;
    }
    if (partial.matrix[0][1] != 0 || partial.matrix[1][0] != 28
        || partial.chars[0][1] != 0 || partial.chars[1][0] != 'n') {
        return 5;
    }
    if (sizeof(frame.matrix) != 6 * sizeof(int)
        || sizeof(frame.chars) != 4 * sizeof(char)
        || sizeof(frame.matrix[1][2]) != sizeof(int)
        || sizeof(slot->chars[0][1]) != sizeof(char)) {
        return 6;
    }

    return 0;
}
