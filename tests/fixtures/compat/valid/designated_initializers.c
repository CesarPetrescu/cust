int global_values[4] = {[2] = 5, [0] = 1};

struct Inner {
    int x;
    int y;
};

struct Packet {
    int id;
    int values[3];
    struct Inner inner;
    char tag;
};

struct Packet global_packet = {
    .inner = {.y = 4, .x = 3},
    .id = 2,
    .values = {[1] = 6, [2] = 7},
    .tag = 'A',
};

int sum_packet(struct Packet packet) {
    return packet.id + packet.values[0] + packet.values[1] + packet.values[2] +
           packet.inner.x + packet.inner.y + packet.tag;
}

int main(void) {
    int local_values[5] = {[3] = 10, [1] = 3, 2};
    struct Packet local = {
        .tag = 2,
        .values = {[0] = 8, [2] = 9},
        .inner = {.x = 1},
        .id = 5,
    };

    return global_values[0] + global_values[2] + sum_packet(global_packet) +
           sum_packet(local) + local_values[1] + local_values[2] + local_values[3];
}
