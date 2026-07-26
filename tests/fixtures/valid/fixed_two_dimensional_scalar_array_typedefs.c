typedef int Matrix[2][3];
typedef char Labels[2][2];
typedef const int ConstMatrix[2][2];

Matrix global = {{1, 2, 3}, {4, 5}};
static Labels file_labels = {{'A'}, {'B', 'C'}};

struct Frame {
    Matrix matrix;
    Labels labels;
};

int touch(void) {
    static Matrix persistent = {{6}, {7, 8}};
    persistent[1][2] += 2;
    return persistent[1][2];
}

int main(void) {
    struct Frame frame = {{{9, 10}, {11}}, {{'D'}, {'E', 'F'}}};
    ConstMatrix fixed = {{12}, {14, 15}};
    int before = frame.matrix[1][0]++;
    global[1][2] = 16;

    if (touch() != 2 || touch() != 4) return 1;
    if (global[0][2] != 3 || global[1][2] != 16) return 2;
    if (file_labels[0][1] != 0 || file_labels[1][1] != 'C') return 3;
    if (before != 11 || frame.matrix[1][0] != 12 || frame.labels[1][0] != 'E') return 4;
    if (fixed[0][1] != 0 || fixed[1][1] != 15) return 5;
    if (sizeof(Matrix) != 6 * sizeof(int)
        || sizeof(Labels) != 4 * sizeof(char)
        || _Alignof(Matrix) != _Alignof(int)
        || _Alignof(Labels) != _Alignof(char)
        || sizeof(frame.matrix) != sizeof(Matrix)
        || sizeof(frame.labels[1][0]) != sizeof(char)) return 6;
    return 0;
}
