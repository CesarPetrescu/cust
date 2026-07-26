typedef int Matrix[2][2];
typedef const int ConstRow[1][2];

int direct_left[1][2] = {{1, 2}}, direct_right[2][1] = {{3}, {4}};
Matrix typed_left = {{5, 6}, {7, 8}}, typed_right = {{9, 10}, {11, 12}};
static char chars_left[1][2] = {{'A', 'B'}}, chars_right[2][1] = {{'C'}, {'D'}};
const Matrix const_left = {{13, 14}, {15, 16}}, const_right = {{17, 18}, {19, 20}};

struct Packet {
    int first[1][2], second[2][1];
    Matrix primary, backup;
};

static struct Packet packet;

int touch(void) {
    static int first[1][2] = {{1, 2}}, second[1][2] = {{3, 4}};
    static Matrix left = {{1, 2}, {3, 4}}, right = {{5, 6}, {7, 8}};
    first[0][0] += 1;
    second[0][1] += 2;
    left[1][0] += 3;
    right[1][1] += 4;
    return first[0][0] + second[0][1] + left[1][0] + right[1][1];
}

int main(void) {
    int local_left[1][2] = {{21, 22}}, local_right[2][1] = {{23}, {24}};
    Matrix local_typed_left = {{25, 26}, {27, 28}}, local_typed_right = {{29, 30}, {31, 32}};
    ConstRow fixed_left = {{33, 34}}, fixed_right = {{35, 36}};

    packet.first[0][1] = 37;
    packet.second[1][0] = 38;
    packet.primary[1][1] = 39;
    packet.backup[0][1] = 40;

    if (touch() != 26 || touch() != 36) return 1;
    if (direct_left[0][1] != 2 || direct_right[1][0] != 4) return 2;
    if (typed_left[1][0] != 7 || typed_right[0][1] != 10) return 3;
    if (chars_left[0][1] != 'B' || chars_right[1][0] != 'D') return 4;
    if (const_left[1][1] != 16 || const_right[1][0] != 19) return 5;
    if (local_left[0][1] != 22 || local_right[1][0] != 24) return 6;
    if (local_typed_left[1][0] != 27 || local_typed_right[1][1] != 32) return 7;
    if (fixed_left[0][1] != 34 || fixed_right[0][0] != 35) return 8;
    if (packet.first[0][1] != 37 || packet.second[1][0] != 38
        || packet.primary[1][1] != 39 || packet.backup[0][1] != 40) return 9;
    if (sizeof(direct_left) != 2 * sizeof(int)
        || sizeof(direct_right) != 2 * sizeof(int)
        || sizeof(typed_left) != sizeof(Matrix)
        || sizeof(local_typed_right) != sizeof(Matrix)
        || sizeof(fixed_left) != 2 * sizeof(int)
        || sizeof(packet.primary) != sizeof(Matrix)) return 10;
    return 0;
}
