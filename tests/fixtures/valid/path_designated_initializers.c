struct Inner {
    int x;
    int y;
};

struct Packet {
    int id;
    int values[4];
    struct Inner inner;
    char tag;
};

struct Packet global_packet = {
    .inner.y = 4,
    .values[2] = 7,
    .id = 3,
    .inner.x = 5,
    .values[0] = 11,
    .tag = 'A',
};

int sum_packet(struct Packet packet) {
    return packet.id + packet.values[0] + packet.values[1] + packet.values[2] +
           packet.values[3] + packet.inner.x + packet.inner.y + packet.tag;
}

int main(void) {
    struct Packet local = {
        .values[1] = 6,
        .inner.x = 8,
        .tag = 2,
        .inner.y = 9,
        .values[3] = 10,
        .id = 12,
    };

    struct Packet mixed = {
        .values[2] = 2,
        .inner.x = 1,
        .inner.y = 5,
        6,
    };

    return sum_packet(global_packet) + sum_packet(local) + sum_packet(mixed);
}
