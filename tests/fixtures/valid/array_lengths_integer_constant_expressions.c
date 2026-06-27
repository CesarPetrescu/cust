enum { DIRECT_LEN = 1 + 2, TYPEDEF_LEN = sizeof "cat" - 1 };

typedef int Scores[TYPEDEF_LEN];

struct Packet {
    int values[DIRECT_LEN];
    char label[TYPEDEF_LEN + 1];
};

int sum_values(int values[1 + 2]) {
    return values[0] + values[1] + values[2];
}

int main(void) {
    int values[DIRECT_LEN] = {1, 2, 3};
    Scores scores = {4, 5, 6};
    struct Packet packet = {{7, 8, 9}, "cat"};
    struct Packet packets[(1 ? 2 : 3)] = {{{1, 1, 1}, "a"}, {{2, 2, 2}, "b"}};
    int *literal = (int[TYPEDEF_LEN]){10, 11, 12};

    return sizeof(values) / sizeof(values[0])
        + sizeof(Scores) / sizeof(scores[0])
        + sizeof(packet.values) / sizeof(packet.values[0])
        + sizeof(packet.label) / sizeof(packet.label[0])
        + sizeof(packets) / sizeof(packets[0])
        + sum_values(values)
        + scores[2]
        + packet.values[1]
        + packets[1].values[2]
        + literal[1];
}
